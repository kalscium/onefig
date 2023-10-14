use flexar::{Flext, token_node::TokenToString};
use crate::{lexer::Token, errors::SyntaxError};

#[derive(Debug)]
pub enum Literal {
    Int(u64),
    Str(String),
    Bool(bool),
}

flexar::parser! {
    [[Literal] parxt: Token]
    parse {
        (Token::Int(x)) => (Int(*x));
        (Token::Bool(x)) => (Bool(*x));
        (Token::Str(x)) => (Str(x.clone()));
    } else Err((SY004, parxt.position()) parxt.current_token());
}