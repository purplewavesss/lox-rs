use std::fmt::{self, Display};
use crate::types::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Statement {
    Block(Box<Vec<Statement>>),
    Expression(Expr),
    Function(Token, Vec<Token>, Box<Vec<Statement>>),
    If(Expr, Box<Statement>, Box<Option<Statement>>),
    Print(Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Statement>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Block(statements) => {
                for stmt in statements.iter() {
                    write!(f, "{stmt}")?;
                }

                Ok(())
            },
            Self::Expression(exp) => write!(f, "{exp}"),
            Self::If(cond, then, els) => match &**els {
                None => write!(f, "{cond} | {} ", *then),
                Some(els) => write!(f, "{cond} | {} | {}", *then, *els)
            }
            Self::Print(exp) => write!(f, "print {exp}"),
            Self::Var(name, _) => write!(f, "{}", name.lexeme),
            Self::While(cond, body) => write!(f, "while {cond} | {body}")
        }
    }
}