use std::fmt;
use enum_as_inner::EnumAsInner;
use crate::types::{callable::LoxCallable, token::Token};

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum Value {
    Identifier(Box<Token>),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Callable(Box<LoxCallable>),
    Nil(),
    None()
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "{name}"),
            Self::Str(string) => write!(f, "{string}"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Bool(bool) => write!(f, "{bool}"),
            Self::Callable(_) => write!(f, "<function>"),
            Self::Nil() => write!(f, ""),
            Self::None() => write!(f, "")
        }
    }
}