use flexar::prelude::Position;
use hashbrown::HashMap;
use crate::{errors::LogicError, conff::ConffType};

#[derive(Debug)]
pub struct ActionTree {
    pub conff_list: Vec<(ConffType, Path, Box<str>)>,
    pub shell_list: Vec<(Path, Box<[Box<str>]>)>,
    pub universal_set: ConfHashMap,
}

impl ActionTree {
    #[inline]
    pub fn new() -> Self {
        Self {
            conff_list: Vec::new(),
            shell_list: Vec::new(),
            universal_set: ConfHashMap::new(),
        }
    }
}

pub type ConfHashMap = HashMap<Box<str>, (Position, Value)>;
pub type Path = Box<[Box<str>]>;

pub trait ConfTable {
    fn set(&mut self, path: &[Box<str>], value: Value, pos: Position);
}

#[derive(Debug)]
pub enum Value {
    String(Box<str>),
    Int(usize),
    Path(Path),
    Bool(bool),
    List(Box<[(Position, Value)]>),
    Table(ConfHashMap),
    Raw(Box<str>),
}

pub trait VisitValue {
    fn visit(self, visitor: &mut ActionTree, scope: &[Box<str>]) -> (Position, Value);
}

pub trait VisitConfig {
    fn visit(self, visitor: &mut ActionTree, map: &mut ConfHashMap, scope: &[Box<str>]);
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