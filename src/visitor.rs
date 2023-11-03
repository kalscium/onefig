use flexar::prelude::Position;
use hashbrown::HashMap;
use crate::{errors::LogicError, conff::ConffType};

#[derive(Debug)]
pub struct ActionTree {
    pub conff_list: Vec<(ConffType, Path, Box<str>)>,
    pub shell_list: Vec<(Path, Box<[Box<str>]>)>,
    pub uni_table: ConfHashMap,
}

impl ActionTree {
    #[inline]
    pub fn new() -> Self {
        Self {
            conff_list: Vec::new(),
            shell_list: Vec::new(),
            uni_table: ConfHashMap::new(),
        }
    }
}

pub type ConfHashMap = HashMap<Box<str>, (Position, Value)>;
pub type Path = Box<[(Position, Box<str>)]>;

pub trait ConfTable {
    fn set(&mut self, path: &[(Position, Box<str>)], value: Value, pos: Position);
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
    fn visit(self, visitor: &mut ActionTree, scope: &[(Position, Box<str>)]) -> (Position, Value);
}

pub trait VisitConfig {
    fn visit(self, visitor: &mut ActionTree, map: &mut ConfHashMap, scope: &[(Position, Box<str>)]);
}

impl ConfTable for ConfHashMap {
    #[inline]
    fn set(&mut self, path: &[(Position, Box<str>)], value: Value, pos: Position) {
        if path.len() == 1 {
            match (self.get_mut(&path[0].1), value) {
                (Some((_, Value::Table(t1))), Value::Table(t2)) => t2.into_iter()
                    .for_each(|(k, (v_pos, v_value))| if let Some(first) = t1.insert(k.clone(), (v_pos.clone(), v_value)) {
                        flexar::compiler_error!((LG001, v_pos) k, first.0.0.ln).throw()
                    }),
                (Some((first, _)), _) => flexar::compiler_error!((LG001, path[0].0.clone()) path[0].1, first.0.ln).throw(),
                (None, value) => {self.insert(path[0].1.clone(), (pos.clone(), value));},
            }; return;
        }

        match self.get_mut(&path[0].1) {
            Some((_, Value::Table(x))) => x.set(&path[1..], value, pos),
            Some((first, _)) => flexar::compiler_error!((LG001, path[0].0.clone()) path[0].1, first.0.ln).throw(),
            None => {
                self.insert(path[0].1.clone(), (pos.clone(), Value::Table(ConfHashMap::new())));
                self.set(path, value, pos);
            },
        }
    }
}