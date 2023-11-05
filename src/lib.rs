use std::ops::Deref;

pub mod lexer;
pub mod errors;
pub mod nodes;
pub mod conff;
pub mod visitor;
pub mod convert;
pub mod target_lang;

#[macro_export]
macro_rules! patt_unwrap {
    (($val:expr) $pat:pat => $out:expr) => {
        match $val {
            $pat => $out,
            _ => panic!("unwrapped on the wrong pattern, expected different pattern"),
        }
    }
}

pub enum MaybeRef<'a, T> { // cause dropped temporary values are annoying
    Ref(&'a T),
    Owned(T),
}

impl<T> Deref for MaybeRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(x) => x,
            Self::Owned(x) => x,
        }
    }
}