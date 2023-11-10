use std::{io::{BufWriter, Result, Write}, fs::{File, self}, path::Path};
use hashbrown::HashMap;
use crate::{visitor::{ConfHashMap, DbgValue, Value}, errors::LogicError};
use flexar::prelude::Position;

#[inline]
pub fn check_table(table: &ConfHashMap) {
    for (_, (pos, x)) in table.iter() {
        check_value(pos, x);
    }
}

#[inline]
pub fn check_value(pos: &Position, value: &DbgValue) {
    match value {
        DbgValue::Table(x) => check_table(x),
        DbgValue::Path(_) => flexar::compiler_error!((LG003, pos.clone()) "path value").throw(),
        DbgValue::List(list) => {
            if list.is_empty() { return };

            let first = &list[0].1;
            for (pos, x) in list.iter() {
                check_value(pos, x);
                if !first.same_type(x) {
                    return flexar::compiler_error!((LG004, pos.clone())).throw();
                }
            }
        },
        _ => (),
    }
}

pub fn generate(path: impl AsRef<Path>, table: &HashMap<Box<str>, Value>) -> Result<()> {
    let _ = fs::create_dir_all(path.as_ref().parent().unwrap_or(&Path::new("")));
    let mut buffer = BufWriter::new(File::create(path)?);
    for (i, (k, x)) in table.iter().enumerate() {
        buffer.write_all(format!("\"{k}\"").as_bytes())?;
        buffer.write_all(b"=")?;
        gen_value(x, &mut buffer)?;
        if i < table.len()-1 {
            buffer.write_all(b"\n")?;
        }
    }
    buffer.flush()
}

#[inline]
fn gen_value(value: &Value, buffer: &mut BufWriter<File>) -> Result<()> {
    use Value as V;
    let value = match value {
        V::Bool(x) => x.to_string(),
        V::Int(x) => x.to_string(),
        V::String(x) => format!("\"{x}\""),
        V::Raw(x) => x.to_string(),
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
        buffer.write_all(k.as_bytes())?; // might not work with custom keys eg `"custom/key": true`
        buffer.write_all(b"=")?; 
        gen_value(x, buffer)?;
        if i < table.len()-1 {
            buffer.write_all(b",")?;
        }
    }
    buffer.write_all(b"}")
}