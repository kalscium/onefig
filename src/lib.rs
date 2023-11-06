pub mod lexer;
pub mod errors;
pub mod nodes;
pub mod conff;
pub mod visitor;
pub mod target_lang;
pub mod cli;

#[macro_export]
macro_rules! patt_unwrap {
    (($val:expr) $pat:pat => $out:expr) => {
        match $val {
            $pat => $out,
            _ => panic!("unwrapped on the wrong pattern, expected different pattern"),
        }
    }
}

#[macro_export]
macro_rules! safe_unwrap {
    ($expr:expr => $type:ident $(,$args:expr)*) => {
        match $expr {
            Ok(x) => x,
            Err(x) => flexar::compiler_error!(($type, flexar::prelude::Position::new_oneline("<runtime>", &x.to_string(), None)) $($args),*).throw(),
        }
    };
}