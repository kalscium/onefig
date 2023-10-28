use flexar::prelude::Position;
use hashbrown::HashMap;
use crate::errors::LogicError;

pub type ConfHashMap = HashMap<Box<str>, (Position, Value)>;
pub trait ConfTable {
    fn set(&mut self, path: &[Box<str>], value: Value, pos: Position);
}

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
    Table(ConfHashMap),
    Raw(Box<str>),
}

pub trait VisitValue {
    fn visit(self, map: &mut ConfHashMap, scope: Box<[Box<str>]>);
}

pub trait VisitConfig {
    fn visit(self, map: &mut ConfHashMap, scope: Box<[Box<str>]>) -> Config;
}

impl ConfTable for ConfHashMap {
    fn set(&mut self, path: &[Box<str>], value: Value, pos: Position) {
        if path.len() == 1 {
            if let Some((first, _)) = self.insert(path[0].clone(), (pos.clone(), value)) {
                flexar::compiler_error!((LG001, pos) path[0], first.0.ln).throw()
            } return;
        }

        match self.get_mut(&path[0]) {
            Some((_, Value::Table(x))) => x.set(&path[1..], value, pos),
            Some((first, _)) => flexar::compiler_error!((LG001, pos) path[0], first.0.ln).throw(),
            None => {
                self.insert(path[0].clone(), (pos.clone(), Value::Table(ConfHashMap::new())));
                self.set(path, value, pos);
            },
        }
    }
}