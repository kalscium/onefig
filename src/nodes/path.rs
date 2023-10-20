use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};

#[derive(Debug)]
pub enum Path {
    Head(Box<str>, Box<Node<Path>>),
    Tail(Box<str>),
}

impl Path {
    pub fn get_head(self) -> Box<str> {
        match self {
            Self::Head(x, _) => x,
            Self::Tail(x) => x,
        }
    }
}

flexar::parser! {
    [[Path] parxt: Token]
    parse {
        [head: Self::path_ident] => {
            (Token::Dot) => {
                [tail: Self::parse] => (Head(head.node.get_head(), Box::new(tail)));
            } (else Err((SY006, parxt.position()) parxt.current_token()))
        } (else Ok(head.node))
    } else Err((SY005, parxt.position()) parxt.current_token());

    parse_w_error {
        (Token::Ident(head)) => {
            (Token::Dot) => {
                [tail: Self::parse] => (Head(head.clone(), Box::new(tail)));
            } (else Err((SY006, parxt.position()) parxt.current_token()))
        } (else Ok(Self::Tail(head.clone())))
    } else Err((SY009, parxt.position()) parxt.current_token(), parxt.current_token());

    path_ident {
        (Token::Ident(x)) => (Tail(x.clone()));
        (Token::Str(x)) => (Tail(x.clone()));
        (Token::Int(x)) => (Tail(x.to_string().into_boxed_str()));
        (Token::Bool(x)) => (Tail(x.to_string().into_boxed_str()));
    } else Err((SY005, parxt.position()) parxt.current_token());
}