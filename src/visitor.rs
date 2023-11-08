use std::{path::{self, PathBuf}, fs};

use flexar::prelude::{Position, Lext};
use hashbrown::HashMap;
use serde::{Serialize, Deserialize};
use crate::{errors::{LogicError, RuntimeError}, conff::ConffType, safe_unwrap, lexer::Token, nodes::source_file::SourceFile};

#[derive(Debug)]
pub struct ActionTree {
    pub conff_list: Vec<(ConffType, Path, Box<str>)>,
    pub shell_list: Vec<(Path, Box<[Box<str>]>)>,
    pub uni_table: ConfHashMap,
    pub included: Vec<(PathBuf, PathBuf)>,
}

impl ActionTree {
    #[inline]
    pub fn new() -> Self {
        Self {
            conff_list: Vec::new(),
            shell_list: Vec::new(),
            included: Vec::new(),
            uni_table: ConfHashMap::new(),
        }
    }

    #[inline]
    pub fn import(&mut self, map: &mut ConfHashMap, path: impl AsRef<path::Path>) {
        let file = safe_unwrap!(fs::read_to_string(&path) => RT007, path.as_ref().to_string_lossy());
        let tokens = Token::tokenize(Lext::new(path.as_ref().to_string_lossy().to_string(), &file));
        let nodes = SourceFile::parse(tokens);
        nodes.0.into_vec()
            .into_iter()
            .for_each(|x| x.visit(self, map, &[]));
    }
}

impl Default for ActionTree {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub type ConfHashMap = HashMap<Box<str>, (Position, DbgValue)>;
pub type Path = Box<[(Position, Box<str>)]>;

pub trait ConfTable {
    fn set(&mut self, path: &[(Position, Box<str>)], value: DbgValue, pos: Position);
}

#[derive(Debug)]
pub enum DbgValue {
    String(Box<str>),
    Int(usize),
    Path(Path),
    Bool(bool),
    List(Box<[(Position, DbgValue)]>),
    Table(ConfHashMap),
    Raw(Box<str>),
}

impl DbgValue {
    pub fn same_type(&self, other: &DbgValue) -> bool {
        macro_rules! double_match {
            ($($pat:pat,)*) => {
                match (self, other) {
                    $(($pat, $pat) => true,)*
                    (DbgValue::Raw(_), _) => true,
                    _ => false,
                }
            }
        }
        
        use DbgValue as D;
        double_match! {
            D::Bool(_),
            D::Int(_),
            D::String(_),
            D::Path(_),
            D::List(_),
            D::Table(_),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    String(Box<str>),
    Int(usize),
    Path(Box<[Box<str>]>),
    Bool(bool),
    List(Box<[Value]>),
    Table(HashMap<Box<str>, Value>),
    Raw(Box<str>),
}

impl From<DbgValue> for Value {
    fn from(value: DbgValue) -> Self {
        use DbgValue as D;
        use Value as V;
        match value {
            D::String(x) => V::String(x),
            D::Int(x) => V::Int(x),
            D::Path(x) => V::Path(x.into_vec().into_iter().map(|(_, x)| x).collect()),
            D::Bool(x) => V::Bool(x),
            D::List(x) => V::List(x.into_vec().into_iter().map(|(_, x)| x.into()).collect()),
            D::Table(x) => V::Table(x.into_iter().map(|(k, (_, x))| (k, x.into())).collect()),
            D::Raw(x) => V::Raw(x),
        }
    }
}

pub trait VisitValue {
    fn visit(self, visitor: &mut ActionTree, scope: &[(Position, Box<str>)]) -> (Position, DbgValue);
}

pub trait VisitConfig {
    fn visit(self, visitor: &mut ActionTree, map: &mut ConfHashMap, scope: &[(Position, Box<str>)]);
}

impl ConfTable for ConfHashMap {
    #[inline]
    fn set(&mut self, path: &[(Position, Box<str>)], value: DbgValue, pos: Position) {
        if path.len() == 1 {
            match (self.get_mut(&path[0].1), value) {
                (Some((_, DbgValue::Table(t1))), DbgValue::Table(t2)) => t2.into_iter()
                    .for_each(|(k, (v_pos, v_value))| if let Some(first) = t1.insert(k.clone(), (v_pos.clone(), v_value)) {
                        flexar::compiler_error!((LG001, v_pos) k, first.0.0.ln).throw()
                    }),
                (Some((first, _)), _) => flexar::compiler_error!((LG001, path[0].0.clone()) path[0].1, first.0.file_name, first.0.ln, first.0.ln_idx).throw(),
                (None, value) => {self.insert(path[0].1.clone(), (pos.clone(), value));},
            }; return;
        }

        match self.get_mut(&path[0].1) {
            Some((_, DbgValue::Table(x))) => x.set(&path[1..], value, pos),
            Some((first, _)) => flexar::compiler_error!((LG001, path[0].0.clone()) path[0].1, first.0.file_name, first.0.ln, first.0.ln_idx).throw(),
            None => {
                self.insert(path[0].1.clone(), (pos.clone(), DbgValue::Table(ConfHashMap::new())));
                self.set(path, value, pos);
            },
        }
    }
}