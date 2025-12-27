use std::{collections::HashMap, fmt::{self, Display}, rc::Rc};
use crate::{types::{token::Token, values::{Value, class::LoxClass}}};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxObject {
    class: Rc<LoxClass>,
    pub fields: HashMap<String, Value>
}

impl LoxObject {
    pub fn new(class: Rc<LoxClass>) -> Self {
        Self { class, fields: HashMap::new() }
    }

    pub fn get_class(&self) -> Rc<LoxClass> {
        self.class.clone()
    }

    pub fn set(&mut self, name: Token, value: Value) {
        self.fields.insert(name.lexeme, value);
    }
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instance of {}", self.class)
    }
}