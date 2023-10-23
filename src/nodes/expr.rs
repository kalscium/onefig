use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};

#[derive(Debug)]
pub enum Expr {}

flexar::parser! {
    [[Expr] parxt: Token]
    parse {
        (LParen) => [compiler_error!((SY404, parxt.position())).throw()];
    } else Err(SY015: parxt.current_token());
}