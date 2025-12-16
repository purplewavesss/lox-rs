use crate::ast::Expr;
use crate::scanning::{token::{Token, TokenValue}, token_type::TokenType::{self, *}};
use thiserror::Error;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Builds ASTs for expressions
    pub fn parse(&mut self) -> Result<Expr, SyntaxError> {
        self.equality()
    }

    /// Builds ASTs for equality
    fn equality(&mut self) -> Result<Expr, SyntaxError> {
        let mut exp: Expr = self.comparison()?;

        while self.match_token(&[NotEqual, EqualEqual]) {
            let op: Token = self.previous();
            let right: Expr = self.comparison()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }

    /// Builds ASTs for comparisons
    fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        let mut exp: Expr = self.term()?;

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op: Token = self.previous();
            let right: Expr = self.term()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }

    /// Builds ASTs for arithmetic expressions
    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut exp: Expr = self.factor()?;

        while self.match_token(&[Minus, Plus]) {
            let op: Token = self.previous();
            let right: Expr = self.factor()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }

    /// Builds ASTs for geometric expressions
    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut exp: Expr = self.unary()?;

        while self.match_token(&[Slash, Asterisk]) {
            let op: Token = self.previous();
            let right: Expr = self.unary()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }
    
    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        if self.match_token(&[Not, Minus]) {
            let op: Token = self.previous();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        match self.tokens[self.current].token_type {
            False => { 
                self.advance();
                Ok(Expr::Literal(TokenValue::Bool(false)))
            },
            True => {
                self.advance();
                Ok(Expr::Literal(TokenValue::Bool(true)))
            },
            Nil => {
                self.advance();
                Ok(Expr::Literal(TokenValue::Nil()))
            },
            Int | Float | Str => {
                self.advance();
                Ok(Expr::Literal(self.previous().literal))
            }
            LeftParen => {
                self.advance();
                let exp: Expr = self.equality()?;
                self.consume(RightParen, "Expect ')' after expression.")?;
                Ok(exp)
            },
            _ => Err(SyntaxError::ParseError(self.tokens[self.current].clone(), String::from("Invalid literal.")))
        }
    }

    /// Checks if token is in list of tokens
    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Checks if token has token type
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *token_type;
    }

    /// Advances the parser forward by one token
    fn advance(&mut self) -> Token{
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    /// Checks if parser is finished
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == End;
    }

    /// Returns next parser token
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Returns previous parser token
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// Consumes a right parenthesis
    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, SyntaxError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(SyntaxError::ParseError(self.peek(), msg.to_string()))
    }

    fn sync(&mut self) {
        self.advance();

        while (!self.is_at_end()) {
            if self.previous().token_type == Semicolon {
                return;
            }

            let _ = match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => ()
            };

            self.advance();
        }
    }
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Syntax error: ")]
    ParseError(Token, String)
}
