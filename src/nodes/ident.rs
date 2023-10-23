use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};

#[derive(Debug)]
pub struct Ident(pub Box<str>);

flexar::parser! {
    [[Ident] parxt: Token]
    parse {
        (Ident(x)) => ((x.clone()));
        (Str(x)) => ((x.clone()));
        (Int(x)) => ((x.to_string().into_boxed_str()));
        (Bool(x)) => ((x.to_string().into_boxed_str()));
    } else Err(SY006: parxt.current_token());
}
