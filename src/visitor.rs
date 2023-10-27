use flexar::prelude::Position;
use hashbrown::HashMap;
use crate::errors::LogicError;

pub type ConfHashMap = HashMap<Box<str>, (Position, Value)>;
pub struct ConfTable(ConfHashMap);

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

    pub fn set(&mut self, path: &[Box<str>], value: Value, pos: Position) {
        if path.len() == 1 {
            if let Some((first, _)) = self.0.insert(path[0].clone(), (pos.clone(), value)) {
                flexar::compiler_error!((LG001, pos) path[0], first.0.ln).throw()
            } return;
        }

        match self.0.get_mut(&path[0]) {
            Some((_, Value::Table(x))) => x.set(&path[1..], value, pos),
            Some((first, _)) => flexar::compiler_error!((LG001, pos) path[0], first.0.ln).throw(),
            None => {
                self.0.insert(path[0].clone(), (pos.clone(), Value::Table(ConfTable::new())));
                self.set(path, value, pos);
            },
        }
    }
}