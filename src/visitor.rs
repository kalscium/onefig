use flexar::prelude::Position;
use hashbrown::HashMap;
use crate::errors::LogicError;

pub struct ConfTable(HashMap<Box<str>, (Position, Value)>);

pub struct Config {
    pub position: Position,
    path: Box<[Box<str>]>,
    value: (Position, Value),
}

pub enum Value {
    String(Box<str>),
    Int(usize),
    Path(Box<[Box<str>]>),
    Bool(bool),
    List(Box<[(Position, Value)]>),
    Table(ConfTable),
    Raw(Box<str>),
}

pub trait VisitValue {
    fn visit(self, map: &mut ConfTable, scope: Box<[Box<str>]>);
}

pub trait VisitConfig {
    fn visit(self, map: &mut ConfTable, scope: Box<[Box<str>]>) -> Config;
}

impl ConfTable {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set(&mut self, path: &mut Box<[Box<str>]>, value: Value, pos: Position) {
        let mut current = &mut self.0;

        for (i, x) in path.iter().enumerate() {
            if i != path.len()-1 {
                if let Some((first, _)) = current.insert(x.clone(), (pos.clone(), Value::Table(ConfTable::new()))) {
                    flexar::compiler_error!((LG001, pos.clone()) x, first.0.ln);
                }
                current = match &mut current.get_mut(x).unwrap().1 {
                    Value::Table(a) => &mut a.0,
                    _ => panic!("not possible"),
                }
            } else {
                if let Some((first, _)) = current.insert(x.clone(), (pos.clone(), value)) {
                    flexar::compiler_error!((LG001, pos) x, first.0.ln);
                } break;
            }
        }
    }
}