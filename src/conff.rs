use std::{path::PathBuf, mem::replace, io::{BufWriter, BufReader, Write}, fs::{File, self}, process::Command};
use flexar::{prelude::*, compile_error::CompileError};
use hashbrown::HashMap;
use serde::{Serialize, Deserialize};
use crate::{safe_unwrap, lexer::Token, errors::{SyntaxError, RuntimeError}, visitor::{ActionTree, DbgValue, Value}, patt_unwrap, target_lang::{json, toml, nix}, recur};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConffTree {
    pub conf_files: Box<[ConfFile]>,
    pub include: Box<[(Box<[u8]>, PathBuf)]>,
}

impl ConffTree {
    /// Gets a boxed slice of conffiles from a `Action Tree` (or att for short)
    pub fn from_att(mut att: ActionTree) -> Self {
        let mut out = Vec::new();

        for (conff_type, path, file_path) in att.conff_list.iter_mut() {
            let file_path = PathBuf::from(file_path.to_string());
            let conff_type = *conff_type;

            // Collect the table
            let table = {
                let mut current = &mut att.uni_table;
                for (_, key) in path.iter() {
                    current = patt_unwrap!((current.get_mut(key)) Some((_, DbgValue::Table(x))) => x); // broken and throws error when `conff toml etc.another: "etc"` fix later
                }

                current.clone()
            };

            // Collect the shell commands
            let shell = {
                let mut shell_cmds = Vec::new();
                for (s_path, cmd) in att.shell_list.iter_mut() {
                    if path.iter().map(|(_, x)| x).collect::<Vec<_>>() == s_path.iter().map(|(_, x)| x).collect::<Vec<_>>() {shell_cmds.push(
                        replace(cmd, Box::new([]))
                    )}
                }
                shell_cmds.into_boxed_slice()
            };

            // Check validity of configs for each target config language
            // (Each config langauge might have different features)
            use ConffType as CFT;
            match conff_type {
                CFT::Json => json::check_table(&table),
                CFT::Toml => toml::check_table(&table),
                CFT::Nix => (), // everything in the language is supported in nix 
            }
            
            out.push(ConfFile {
                conff_type,
                table: table.into_iter().map(|(k, (_, x))| (k, x.into())).collect(),
                path: file_path,
                shell,
            })
        }

        // Collect Included files recursively
        let mut include = Vec::new();
        for (path, target) in att.included.into_iter() {
            recur! {
                walk_dir(path: PathBuf, target: PathBuf, include: &mut Vec<(Box<[u8]>, PathBuf)>) <- (path, target, &mut include) {
                    if path.is_dir() {
                        let dir = safe_unwrap!(fs::read_dir(&path) => RT014, path.to_string_lossy());
                        for item in dir {
                            let item = safe_unwrap!(item => RT015, path.to_string_lossy());
                            walk_dir(item.path(), target.join(item.file_name()), include);
                        }
                    } else {
                        include.push((lz4_flex::block::compress_prepend_size(safe_unwrap!(fs::read_to_string(&path) => RT008, path.to_string_lossy()).as_bytes()).into_boxed_slice(), target));
                    }
                }
            }
        }

        Self {
            conf_files: out.into_boxed_slice(),
            include: include.into_boxed_slice(),
        }
    }

    pub fn compile(&self, path: impl AsRef<std::path::Path>) { // todo: implement better error handling
        let buffer = BufWriter::new(safe_unwrap!(File::create(&path) => RT003, path.as_ref().to_string_lossy()));
        safe_unwrap!(bincode::serialize_into(buffer, self) => RT003, path.as_ref().to_string_lossy());
    }

    pub fn load_compiled(path: impl AsRef<std::path::Path>) -> Self { // todo: implement better error handling
        let buffer = BufReader::new(safe_unwrap!(File::open(&path) => RT005, path.as_ref().to_string_lossy()));
        safe_unwrap!(bincode::deserialize_from(buffer) => RT002, path.as_ref().to_string_lossy())
    }

    /// Generates the configuration files
    pub fn generate(&self) {
        // Generate config files
        self.conf_files.iter()
            .for_each(|x| x.generate());

        // Copy the included files
        for (contents, path) in self.include.iter() {
            safe_unwrap!(fs::write(path,
                safe_unwrap!(lz4_flex::block::decompress_size_prepended(contents)
                    => RT013, path.to_string_lossy()
                )) => RT009, path.to_string_lossy()
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfFile {
    pub conff_type: ConffType,
    pub table: HashMap<Box<str>, Value>,
    pub path: PathBuf,
    pub shell: Box<[Box<[Box<str>]>]>,
}

impl ConfFile {
    pub fn generate(&self) { // todo: proper errors and handling of such
        use ConffType as C;
        match self.conff_type {
            C::Json => safe_unwrap!(json::generate(&self.path, &self.table) => RT006, self.path.to_string_lossy()),
            C::Toml => safe_unwrap!(toml::generate(&self.path, &self.table) => RT006, self.path.to_string_lossy()),
            C::Nix => safe_unwrap!(nix::generate(&self.path, &self.table) => RT006, self.path.to_string_lossy()),
        }
        self.execute_shell();
    }

    pub fn execute_shell(&self) { // todo: proper errors and handling of such
        for cmd in self.shell.iter() {
            let cmd_display = cmd.join(" ");

            // Ask user if they confirm
            let mut user_out = String::new();
            println!("\n{}", flexar::colour_format![
                cyan("Are you sure you want to run this cmd `"),
                none(&cmd_display),
                cyan("`?\n"),
                blue("("),
                yellow("rewrite the command below"),
                blue(")"),
            ]);
            print!("{}", flexar::colour_format![blue("-> ")]);
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut user_out).unwrap();

            // Check if the user wrote correctly
            if user_out.trim() != cmd_display.trim() {
                println!("{}", flexar::colour_format![
                    cyan("Shell commands "),
                    red("do not"),
                    cyan(" match, "),
                    yellow("skipping"),
                    cyan("...\n")
                ]);
                continue;
            }

            // Execute and use stdout
            println!("{}", flexar::colour_format![
                blue("\n==="),
                cyan(" shell cmd `"),
                none(&cmd_display),
                cyan("` stdout start "),
                blue("==="),
            ]);

            let status = safe_unwrap!(Command::new(cmd[0].as_ref())
                .args(cmd[1..].iter().map(|x| x.as_ref()))
                .status() => RT004, cmd_display);
            if !status.success() {
                return flexar::compiler_error!((RT001, Position::new_oneline("<shell>", &cmd_display, None))).throw();
            }

            println!("{}", flexar::colour_format![
                blue("==="),
                cyan(" shell cmd `"),
                none(&cmd_display),
                cyan("` stdout end   "),
                blue("===\n"),
            ]);
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConffType {
    Toml,
    Json,
    Nix,
}

impl ConffType {
    pub fn parse(parxt: &mut Parxt<'_, Token>) -> Result<Node<Self>, (u8, CompileError)> {
        if let Some(Token::Ident(ident)) = parxt.current() {
            let out = Ok(
                Node { position: parxt.position(), node: match ident.as_ref() {
                    "toml" => Self::Toml,
                    "json" => Self::Json,
                    "nix" => Self::Nix,
                    _ => return Err((1, compiler_error!((SY017, parxt.position()) parxt.current_token()))),
                }
            });
            parxt.advance();
            return out;
        }

        Err((0, compiler_error!((SY018, parxt.position()) parxt.current_token())))
    }
}