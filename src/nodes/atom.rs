use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::{path::Path, list::List};

#[derive(Debug)]
pub enum Atom {
    Int(usize),
    Str(Box<str>),
    Bool(bool),
    Path(Node<Path>),
    List(Box<Node<List>>),
}

flexar::parser! {
    [[Atom] parxt: Token]
    parse {
        [list: List::parse] => (List(Box::new(list)));
        [path: Path::parse_w_error] => (Path(path));
        (Token::Int(x)) => (Int(*x));
        (Token::Bool(x)) => (Bool(*x));
        (Token::Str(x)) => (Str(x.clone()));
    } else Err((SY004, parxt.position()) parxt.current_token());
}