use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::stmt::Stmt;

#[derive(Debug)]
pub enum Table {
    Head(Node<Stmt>, Box<Node<Table>>),
    Tail(Node<Stmt>),
    Empty,
}

flexar::parser! {
    [[Table] parxt: Token]
    parse {
        (LBrace) => {
            (RBrace) => (Empty);
            [item: Self::table_item] => {
                (RBrace) => [item];
            } (else Err(SY014: parxt.current_token()))
        };
    } else Err(SY013: parxt.current_token());

    table_item {
        [head: Stmt::parse] => {
            (Sep(_)) => {
                [tail: Self::table_item] => (Head(head, Box::new(tail)));
            } (else Ok(Self::Tail(head)))
            [tail: Self::table_item] => (Head(head, Box::new(tail)));
        } (else Ok(Self::Tail(head)))
    } else Err(SY014: parxt.current_token());
}