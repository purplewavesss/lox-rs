use std::{fmt::{self, Display}, rc::Rc};
use crate::types::values::class::LoxClass;

#[derive(Clone, Debug, PartialEq)]
pub struct LoxObject {
    class: Rc<LoxClass>
}

impl LoxObject {
    pub fn new(class: Rc<LoxClass>) -> Self {
        Self { class }
    }
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instance of {}", self.class.name)
    }
}