use std::fmt::{self, Display};
use crate::{expr::Expr, scanning::token::Token};

#[derive(Clone, Debug)]
pub enum Statement {
    Block(Vec<Box<Statement>>),
    Expression(Expr),
    If(Expr, Box<Statement>, Box<Option<Statement>>),
    Print(Expr),
    Var(Token, Option<Expr>)
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Block(statements) => {
                for stmt in statements {
                    write!(f, "{stmt}")?;
                }

                Ok(())
            }
            Self::Expression(exp) => write!(f, "{exp}"),
            Self::If(cond, then, els) => match &**els {
                None => write!(f, "{cond} | {} ", *then),
                Some(els) => write!(f, "{cond} | {} | {}", *then, *els)
            }
            Self::Print(exp) => write!(f, "print {exp}"),
            Self::Var(name, _) => write!(f, "{}", name.lexeme)
        }
    }
}