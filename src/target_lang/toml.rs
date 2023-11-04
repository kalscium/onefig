use crate::{visitor::{ConfHashMap, DbgValue}, errors::LogicError};

#[inline]
pub fn check_table(table: &ConfHashMap) {
    for (_, (pos, x)) in table.iter() {
        match x {
            DbgValue::Table(x) => check_table(x),
            DbgValue::Path(_) => flexar::compiler_error!((LG003, pos.clone()) "path value").throw(),
            _ => (),
        }
    }
}