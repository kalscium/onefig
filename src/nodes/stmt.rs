use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError, conff::ConffType, visitor::{VisitConfig, ActionTree, ConfHashMap, VisitValue, ConfTable}};
use super::{path::Path, atom::Atom};

#[derive(Debug)]
pub enum Stmt {
    Config(Node<Path>, Node<Atom>),
    Shell(Node<Path>, Box<[Box<str>]>),
    Conff {
        conff_type: ConffType,
        path: Node<Path>,
        file_path: Node<Atom>,
    },
    Var(Node<Path>),
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse {
        [path: Path::parse] => {
            (Shell(shell)) => (Shell(path, shell.clone()));
            (Set(_)), [atom: Atom::parse] => (Config(path, atom));
        } (else Err(SY008: parxt.current_token()))

        (Conff), [conff_type: ConffType::parse], [path: Path::parse] => {
            (Set(_)), [atom: Atom::parse] => (Conff {
                conff_type: conff_type.node,
                path,
                file_path: atom,
            });
        } (else Err(SY008: parxt.current_token()))

        (Var), [path: Path::parse] => (Var(path));
    } else Err(SY007: parxt.current_token());
}

impl VisitConfig for Node<Stmt> {
    fn visit(self, visitor: &mut ActionTree, map: &mut ConfHashMap, scope: &[(Position, Box<str>)]) {
        use Stmt as S;
        match self.node {
            S::Var(_) => (), // varibles will just be dropped for now
            S::Shell(path, cmd) => visitor.shell_list.push((Path::flatten(path), cmd)),
            S::Conff { // Config File
                conff_type,
                path,
                file_path
            } => visitor.conff_list.push((conff_type, Path::flatten(path), match file_path.node {
                Atom::Str(x) => x,
                _ => compiler_error!((SY404, self.position)).throw(),
            })),
            S::Config(path, value) => {let atom = value.visit(visitor, scope); map.set(
                &Path::flatten(path),
                atom.1,
                atom.0
            )},
        }
    }
}