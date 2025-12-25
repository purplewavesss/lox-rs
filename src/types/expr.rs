use strum_macros::Display;
use crate::types::{token::Token, values::Value};

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Expr {
    // e.g. "5" or "test"
    Literal(Value),
    // e.g. -1 or !bool
    Unary(Token, Box<Expr>),
    // e.g. 5 + 2
    Binary(Box<Expr>, Token, Box<Expr>),
    // e.g. (8 * 5 - 1)
    Grouping(Box<Expr>),
    // e.g. true or false
    Logical(Box<Expr>, Token, Box<Expr>),
    // e.g. function()
    Call(Box<Expr>, Box<Vec<Expr>>),
    // e.g. a
    Variable(Token),
    // e.g. a = 5
    Assign(Token, Box<Expr>)
}