use std::{io::{BufWriter, Result, Write}, fs::File, path::Path};
use hashbrown::HashMap;

use crate::{visitor::{ConfHashMap, DbgValue, Value}, errors::LogicError, MaybeRef};

#[inline]
pub fn check_table(table: &ConfHashMap) {
    for (_, (pos, x)) in table.iter() {
        match x {
            DbgValue::Table(x) => check_table(x),
            DbgValue::Path(_) => flexar::compiler_error!((LG002, pos.clone()) "path value").throw(),
            _ => (),
        }
    }
}

pub fn generate(path: impl AsRef<Path>, table: &HashMap<Box<str>, Value>) -> Result<()> {
    let mut buffer = BufWriter::new(File::create(path)?);
    gen_table(table, &mut buffer)?;
    buffer.flush()
}

#[inline]
fn gen_value(value: &Value, buffer: &mut BufWriter<File>) -> Result<()> {
    use Value as V;
    use MaybeRef as M;
    let value: MaybeRef<Box<str>> = match value {
        V::Bool(x) => M::Owned(x.to_string().into_boxed_str()),
        V::Int(x) => M::Owned(x.to_string().into_boxed_str()),
        V::String(x) => M::Owned(format!("\"{x}\"").into_boxed_str()),
        V::Raw(x) => M::Ref(x),
        V::Path(_) => panic!("shouldn't happen"), // propper error should occur way before this
        V::List(x) => return gen_list(x, buffer),
        V::Table(x) => return gen_table(x, buffer),
    };
    buffer.write_all(value.as_bytes())
}

#[inline]
fn gen_list(list: &[Value], buffer: &mut BufWriter<File>) -> Result<()> {
    buffer.write_all(b"[")?;
    for (i, x) in list.iter().enumerate() {
        gen_value(x, buffer)?;
        if i < list.len()-1 {
            buffer.write_all(b",")?;
        }
    }
    buffer.write_all(b"]")
}

#[inline]
fn gen_table(table: &HashMap<Box<str>, Value>, buffer: &mut BufWriter<File>) -> Result<()> {
    buffer.write_all(b"{")?;
    for (i, (k, x)) in table.iter().enumerate() {
        buffer.write_all(format!("\"{k}\"").as_bytes())?;
        buffer.write_all(b":")?; 
        gen_value(x, buffer)?;
        if i < table.len()-1 {
            buffer.write_all(b",")?;
        }
    }
    buffer.write_all(b"}")
}