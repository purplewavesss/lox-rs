use crate::expr::Expr;

pub enum Statement {
    Expression(Expr),
    Print(Expr)
}