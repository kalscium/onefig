use crate::{visitor::{ConfHashMap, Value}, errors::LogicError};

#[inline]
pub fn check_table(table: &ConfHashMap) {
    for (_, (pos, x)) in table.iter() {
        match x {
            Value::Table(x) => check_table(x),
            Value::Path(_) => flexar::compiler_error!((LG002, pos.clone()) "path value").throw(),
            _ => (),
        }
    }
}