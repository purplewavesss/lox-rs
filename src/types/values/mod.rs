use std::fmt;
use std::rc::Rc;
use enum_as_inner::EnumAsInner;
use crate::types::values::object::LoxObject;
use crate::types::values::{callable::LoxCallable, class::LoxClass};
use crate::types::token::Token;

pub mod callable;
pub mod class;
pub mod object;

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum Value {
    Identifier(Box<Token>),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Callable(LoxCallable),
    Class(Rc<LoxClass>),
    Instance(LoxObject),
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
            Self::Callable(callable) => callable.fmt(f),
            Self::Class(class) => class.fmt(f),
            Self::Instance(obj) => obj.fmt(f),
            Self::Nil() => write!(f, ""),
            Self::None() => write!(f, "")
        }
    }
}