use flexar::prelude::*;
use crate::lexer::Token;
use super::stmt::Stmt;

#[derive(Debug)]
pub struct SourceFile(Box<[Node<Stmt>]>);

impl SourceFile {
    pub fn parse(tokens: &[token_node::Token<Token>]) -> Option<Self> {
        if tokens.is_empty() { return None }

        let mut parxt = Parxt::new(tokens);
        let mut stmts = Vec::new();

        while parxt.current().is_some() {
            stmts.push(match Stmt::parse(&mut parxt) {
                Ok(x) => x,
                Err((_, x)) => x.throw(),
            });
        }

        Some(Self(stmts.into_boxed_slice()))
    }
}