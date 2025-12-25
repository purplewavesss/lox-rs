use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxClass {
    name: String
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}