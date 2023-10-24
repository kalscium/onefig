use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError, conff::ConffType};
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
    RawConf(Box<str>),
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse {
        (RawConf(x)) => (RawConf(x.clone()));

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