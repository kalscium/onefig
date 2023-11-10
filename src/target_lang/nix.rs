use std::{io::{BufWriter, Result, Write}, fs::{File, self}, path::Path};
use hashbrown::HashMap;
use crate::visitor::Value;

pub fn generate(path: impl AsRef<Path>, table: &HashMap<Box<str>, Value>) -> Result<()> {
    let _ = fs::create_dir_all(path.as_ref().parent().unwrap_or(Path::new("")));
    let mut buffer = BufWriter::new(File::create(path)?);
    buffer.write_all(b"{config,pkgs,...}:")?;
    gen_table(table, &mut buffer)?;
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
        V::Path(x) => x.join("."),
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
            buffer.write_all(b" ")?;
        }
    }
    buffer.write_all(b"]")
}

#[inline]
fn gen_table(table: &HashMap<Box<str>, Value>, buffer: &mut BufWriter<File>) -> Result<()> {
    buffer.write_all(b"{")?;
    for (k, x) in table.iter() {
        buffer.write_all(format!("\"{k}\"").as_bytes())?;
        buffer.write_all(b"=")?; 
        gen_value(x, buffer)?;
        buffer.write_all(b";")?;
    }
    buffer.write_all(b"}")
}