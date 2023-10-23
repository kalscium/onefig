use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError};
use super::{path::Path, list::List, table::Table, expr::Expr};

#[derive(Debug)]
pub enum Atom {
    Int(usize),
    Str(Box<str>),
    Bool(bool),
    Path(Node<Path>),
    List(Box<Node<List>>),
    Table(Box<Node<Table>>),
    Expr(Box<Node<Expr>>),
}

flexar::parser! {
    [[Atom] parxt: Token]
    parse {
        [expr: Expr::parse] => (Expr(Box::new(expr)));
        [table: Table::parse] => (Table(Box::new(table)));
        [list: List::parse] => (List(Box::new(list)));
        [path: Path::parse_w_error] => (Path(path));
        (Int(x)) => (Int(*x));
        (Bool(x)) => (Bool(*x));
        (Str(x)) => (Str(x.clone()));
    } else Err(SY004: parxt.current_token());
}