use std::path::PathBuf;

use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError, conff::ConffType};
use super::{path::Path, atom::Atom, ident::Ident};

#[derive(Debug)]
pub enum Stmt {
    Config(Node<Path>, Node<Atom>),
    Shell(Node<Path>, Box<[Box<str>]>),
    Conff {
        conff_type: ConffType,
        ident: Box<str>,
        path: Node<Atom>,
    },
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse {
        [path: Path::parse] => {
            (Shell(shell)) => (Shell(path, shell.clone()));
            (Set(_)), [atom: Atom::parse] => (Config(path, atom));
        } (else Err(SY008: parxt.current_token()))

        (Conff), [conff_type: ConffType::parse], [ident: Ident::parse] => {
            (Set(_)), [atom: Atom::parse] => (Conff {
                conff_type: conff_type.node,
                ident: ident.node.0,
                path: atom,
            });
        } (else Err(SY008: parxt.current_token()))
    } else Err(SY007: parxt.current_token());
}