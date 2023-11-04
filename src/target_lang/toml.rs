use crate::{visitor::{ConfHashMap, Value}, errors::LogicError};

pub fn check_table(table: &ConfHashMap) {
    for (_, (pos, x)) in table.iter() {
        if let Value::Path(_) = x {
            flexar::compiler_error!((LG003, pos.clone()) "path value").throw()
        }
    }
}