use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::path::Path;

#[derive(Debug)]
pub enum Atom {
    Int(usize),
    Str(String),
    Bool(bool),
    Path(Node<Path>),
}

flexar::parser! {
    [[Atom] parxt: Token]
    parse {
        [path: Path::parse_w_error] => (Path(path));
        (Token::Int(x)) => (Int(*x));
        (Token::Bool(x)) => (Bool(*x));
        (Token::Str(x)) => (Str(x.clone()));
    } else Err((SY004, parxt.position()) parxt.current_token());
}