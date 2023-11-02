use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError, visitor::{VisitValue, Value, ActionTree}};
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
        (LBracket) => {
            (RBracket) => (Empty);
            [item: Self::list_item] => {
                (RBracket) => [item];
            } (else Err(SY011: parxt.current_token()))
        };
    } else Err(SY010: parxt.current_token());

    list_item {
        [head: Atom::parse] => {
            [tail: Self::list_item] => (Head(head, Box::new(tail)));
            (Sep(_)) => {
                [tail: Self::list_item] => (Head(head, Box::new(tail)));
            } (else Ok(Self::Tail(head)))
        } (else Ok(Self::Tail(head)))
    } else Err(SY011: parxt.current_token());
}

impl VisitValue for Node<List> {
    fn visit(self, visitor: &mut ActionTree, scope: &[(Position, Box<str>)]) -> (Position, Value) {
        let mut current = self.node;
        let mut out = Vec::new();
        loop {
            match current {
                List::Empty => break,
                List::Tail(x) => { out.push(x.visit(visitor, scope)); break },
                List::Head(x, y) => {
                    out.push(x.visit(visitor, scope));
                    current = y.node;
                },
            }
        }

        (self.position, Value::List(out.into_boxed_slice()))
    }
}