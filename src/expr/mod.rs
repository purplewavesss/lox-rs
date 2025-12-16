use std::fmt::{self, Display};
use crate::scanning::token::{Token, Value};

#[derive(Clone, Debug)]
pub enum Expr {
    // e.g. "5" or "test"
    Literal(Value),
    // e.g. -1 or !bool
    Unary(Token, Box<Expr>),
    // e.g. 5 + 2
    Binary(Box<Expr>, Token, Box<Expr>),
    // e.g. (8 * 5 - 1)
    Grouping(Box<Expr>)
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(token) => write!(f, "{token}"),
            Expr::Unary(op, literal) => write!(f, "({} {literal})", op.lexeme),
            Expr::Binary(l, op, r) => write!(f, "({} {l} {r})", op.lexeme),
            Expr::Grouping(exp) => write!(f, "(group {exp})")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanning::token_type::TokenType;
    use super::*;

    #[test]
    fn matches_book_example() {
        let exp = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, '-'.to_string(), Value::None(), 1), 
                Box::new(Expr::Literal(Value::Int(123)))
            )),
            Token::new(TokenType::Asterisk, '*'.to_string(), Value::None(), 1),
            Box::new(Expr::Grouping(
                Box::new(
                    Expr::Literal(Value::Float(45.67))
                )
            ))
        );

        assert_eq!(exp.to_string(), "(* (- 123) (group 45.67))".to_string())
    }
}