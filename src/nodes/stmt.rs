use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::{path::Path, atom::Atom};

#[derive(Debug)]
pub enum Stmt {
    Config(Node<Path>, Node<Atom>),
    Shell(Node<Path>, Box<[Box<str>]>),
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse {
        [path: Path::parse] => {
            (Token::Shell(shell)) => (Shell(path, shell.clone()));
            (Token::Set(_)), [atom: Atom::parse] => {
                (Token::Sep(_)) => (Config(path, atom));
            } (else Ok(Self::Config(path, atom)))
        } (else Err((SY008, parxt.position()) parxt.current_token()))
    } else Err((SY007, parxt.position()) parxt.current_token());
}