use flexar::{Flext, token_node::{TokenToString, Node}};
use crate::{lexer::Token, errors::SyntaxError};

#[derive(Debug)]
pub enum Path {
    More(String, Box<Node<Path>>),
    Tail(String),
}

impl Path {
    pub fn get_head(self) -> String {
        match self {
            Self::More(x, _) => x,
            Self::Tail(x) => x,
        }
    }
}

flexar::parser! {
    [[Path] parxt: Token]
    parse {
        [head: Self::path_ident] => {
            (Token::Dot) => {
                [tail: Self::parse] => (More(head.node.get_head(), Box::new(tail)));
            } (else Err((SY006, parxt.position()) parxt.current_token()))
        } (else Ok(head.node))
    } else Err((SY005, parxt.position()) parxt.current_token());

    path_ident {
        (Token::Ident(x)) => (Tail(x.clone()));
        (Token::Str(x)) => (Tail(x.clone()));
        (Token::Int(x)) => (Tail(x.to_string()));
    } else Err((SY005, parxt.position()) parxt.current_token());
}