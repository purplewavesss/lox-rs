use std::{collections::HashMap, fmt::{self, Display}, rc::Rc};
use crate::{LoxError, types::{token::Token, values::{Value, class::LoxClass}}};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxObject {
    class: Rc<LoxClass>,
    fields: HashMap<String, Value>
}

impl LoxObject {
    pub fn new(class: Rc<LoxClass>) -> Self {
        Self { class, fields: HashMap::new() }
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        if self.fields.contains_key(&name.lexeme) {
            Ok(self.fields.get(&name.lexeme).unwrap().clone())
        }

        else {
            match self.class.find_method(&name.lexeme) {
                Some(method) => {
                    Ok(Value::Callable(method.clone().bind(&self)))
                },

                None => {
                    let token_name = name.lexeme.clone();
                    Err(LoxError::NameError(token_name, format!("Undefined property {}", name.lexeme)))
                }
            }
        }
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