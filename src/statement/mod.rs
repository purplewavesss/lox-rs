use std::fmt::{self, Display};
use crate::{expr::Expr, scanning::token::Token};

#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>)
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Expression(exp) => write!(f, "{exp}"),
            Self::Print(exp) => write!(f, "print {exp}"),
            Self::Var(name, _) => write!(f, "{}", name.lexeme)
        }
    }
}