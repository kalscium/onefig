use std::{path::PathBuf, mem::replace};
use flexar::{prelude::*, compile_error::CompileError};
use crate::{lexer::Token, errors::SyntaxError, visitor::{ConfHashMap, ActionTree, Value}, patt_unwrap};

#[derive(Debug)]
pub struct ConfFile {
    pub conff_type: ConffType,
    pub table: ConfHashMap,
    pub path: PathBuf,
    pub shell: Box<[Box<[Box<str>]>]>,
}

impl ConfFile {
    /// Gets a boxed slice of conffiles from a `Action Tree` (or att for short)
    pub fn from_att(mut att: ActionTree) -> Box<[Self]> {
        let mut out = Vec::new();

        for (conff_type, path, file_path) in att.conff_list.iter_mut() {
            let file_path = PathBuf::from(file_path.to_string());

            // Collect the table
            let table = {
                let mut current = &mut att.uni_table;
                for (_, key) in path.iter() {
                    current = patt_unwrap!((current.get_mut(key)) Some((_, Value::Table(x))) => x); // if not a table, it should throw an error way before this
                }

                replace(current, ConfHashMap::new()) // move out of reference
            };

            // Collect the shell commands
            let shell = {
                let mut shell_cmds = Vec::new();
                for (s_path, cmd) in att.shell_list.iter_mut() {
                    if path == s_path {shell_cmds.push( // eq operation might work..?
                        replace(cmd, Box::new([]))
                    )}
                }
                shell_cmds.into_boxed_slice()
            };
            
            out.push(ConfFile {
                conff_type: replace(conff_type, ConffType::Json),
                table,
                path: file_path,
                shell,
            })
        }

        out.into_boxed_slice()
    }
}

#[derive(Debug, Clone, Copy)]
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