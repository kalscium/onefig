pub mod lexer;
pub mod errors;
pub mod nodes;
pub mod conff;
pub mod visitor;
pub mod convert;

#[macro_export]
macro_rules! patt_unwrap {
    (($val:expr) $pat:pat => $out:expr) => {
        match $val {
            $pat => $out,
            _ => panic!("unwrapped on the wrong pattern, expected different pattern"),
        }
    }
}