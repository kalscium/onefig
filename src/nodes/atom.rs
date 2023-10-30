use flexar::prelude::*;
use crate::{lexer::Token, errors::SyntaxError, visitor::{VisitValue, Value}};
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
    Apply(Node<Path>, Box<Node<Atom>>),
    RawConf(Box<str>),
}

flexar::parser! {
    [[Atom] parxt: Token]
    parse {
        [expr: Expr::parse] => (Expr(Box::new(expr)));
        [table: Table::parse] => (Table(Box::new(table)));
        [list: List::parse] => (List(Box::new(list)));
        [path: Path::parse_w_error] => {
            (Apply), [atom: Atom::parse] => (Apply(path, Box::new(atom)));
        } (else Ok(Self::Path(path)))
        (Int(x)) => (Int(*x));
        (Bool(x)) => (Bool(*x));
        (Str(x)) => (Str(x.clone()));
        (RawConf(x)) => (RawConf(x.clone()));
    } else Err(SY004: parxt.current_token());
}

impl VisitValue for Node<Atom> {
    fn visit(self, _: &[Box<str>]) -> (Position, Value) {
        todo!()
    }
}

// impl VisitValue for Node<Atom> {
//     fn visit(self, scope: Vec<Box<str>>) -> (Position, Value) {
//         use Atom as A;
//         use Value as V;
//         (self.position, match self.node {
//             A::Int(x) => V::Int(x),
//             A::Str(x) => V::String(x),
//             A::Bool(x) => V::Bool(x),
//             A::RawConf(x) => V::Raw(x),
//             // A::List(x) => V::List(),
//             _ => todo!(),
//         })
//     }
// }