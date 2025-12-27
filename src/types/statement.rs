use std::fmt::{self, Display};
use enum_as_inner::EnumAsInner;
use crate::types::{expr::Expr, token::Token};

#[derive(Clone, Debug, PartialEq, EnumAsInner)]
pub enum Statement {
    Block(Box<Vec<Statement>>),
    Class(Token, Box<Vec<Statement>>),
    FunDeclaration(Token, Vec<Token>, Box<Vec<Statement>>),
    Expression(Expr),
    If(Expr, Box<Statement>, Box<Option<Statement>>),
    Print(Expr),
    Return(Expr),
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
            Self::Class(name, _) => write!(f, "{name}"),
            Self::FunDeclaration(name, _, _) => write!(f, "{}", name.lexeme),
            Self::Expression(exp) => write!(f, "{exp}"),
            Self::If(cond, then, els) => match &**els {
                None => write!(f, "{cond} | {} ", *then),
                Some(els) => write!(f, "{cond} | {} | {}", *then, *els)
            }
            Self::Print(exp) => write!(f, "print {exp}"),
            Self::Return(exp) => write!(f, "return {exp}"),
            Self::Var(name, _) => write!(f, "{}", name.lexeme),
            Self::While(cond, body) => write!(f, "while {cond} | {body}")
        }
    }
}