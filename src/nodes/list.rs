use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::atom::Atom;

#[derive(Debug)]
pub enum List {
    Head(Node<Atom>, Box<Node<List>>),
    Tail(Node<Atom>),
    Empty,
}

flexar::parser! {
    [[List] parxt: Token]
    parse {
        (Token::LBracket) => {
            (Token::RBracket) => (Empty);
            [item: Self::list_item] => {
                (Token::RBracket) => [item];
            } (else Err((SY011, parxt.position()) parxt.current_token()))
        };
    } else Err((SY010, parxt.position()) parxt.current_token());

    list_item {
        [head: Atom::parse] => {
            [tail: Self::list_item] => (Head(head, Box::new(tail)));
            (Token::Sep(_)) => {
                [tail: Self::list_item] => (Head(head, Box::new(tail)));
            } (else Ok(Self::Tail(head)))
        } (else Ok(Self::Tail(head)))
    } else Err((SY011, parxt.position()) parxt.current_token());
}