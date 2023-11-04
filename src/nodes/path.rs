use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::ident::Ident;

#[derive(Debug, Clone)]
pub enum Path {
    Head(Box<str>, Box<Node<Path>>),
    Tail(Box<str>),
}

impl Path {
    pub fn flatten(mut this: Node<Path>) -> Box<[(Position, Box<str>)]> {
        let mut out = Vec::new();
        loop {
            match this.node {
                Path::Tail(x) => { out.push((this.position, x)); break },
                Path::Head(x, y) => {
                    out.push((this.position, x));
                    this = *y;
                }
            }
        }

        out.into_boxed_slice()
    }
}

flexar::parser! {
    [[Path] parxt: Token]
    parse {
        [head: Ident::parse] => {
            (Dot) => {
                [tail: Self::parse] => (Head(head.node.0, Box::new(tail)));
            } (else Err(SY006: parxt.current_token()))
        } (else Ok(Self::Tail(head.node.0)))
    } else Err(SY005: parxt.current_token());

    parse_w_error {
        (Ident(head)) => {
            (Dot) => {
                [tail: Self::parse] => (Head(head.clone(), Box::new(tail)));
            } (else Err(SY006: parxt.current_token()))
        } (else Ok(Self::Tail(head.clone())))
    } else Err(SY009: parxt.current_token(), parxt.current_token());

    path_ident {
        (Ident(x)) => (Tail(x.clone()));
        (Str(x)) => (Tail(x.clone()));
        (Int(x)) => (Tail(x.to_string().into_boxed_str()));
        (Bool(x)) => (Tail(x.to_string().into_boxed_str()));
    } else Err(SY009: parxt.current_token(), parxt.current_token());
}