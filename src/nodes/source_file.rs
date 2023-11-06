use flexar::prelude::*;
use crate::{lexer::Token, visitor::{ActionTree, VisitConfig, ConfHashMap}};
use super::stmt::Stmt;

#[derive(Debug)]
pub struct SourceFile(pub Box<[Node<Stmt>]>);

impl SourceFile {
    pub fn parse(tokens: Box<[token_node::Token<Token>]>) -> Self {
        if tokens.is_empty() { return Self(Box::new([])) }

        let mut parxt = Parxt::new(&tokens);
        let mut stmts = Vec::new();

        while parxt.current().is_some() {
            stmts.push(match Stmt::parse(&mut parxt) {
                Ok(x) => x,
                Err((_, x)) => x.throw(),
            });
            if let Some(Token::Sep(_)) = parxt.current() { parxt.advance() };
        }

        Self(stmts.into_boxed_slice())
    }

    pub fn visit(self) -> ActionTree {
        let mut visitor = ActionTree::new();
        let mut uni_table = ConfHashMap::new();

        for i in self.0.into_vec().into_iter() {
            i.visit(&mut visitor, &mut uni_table, &[])
        }

        visitor.uni_table = uni_table;
        visitor
    }
}