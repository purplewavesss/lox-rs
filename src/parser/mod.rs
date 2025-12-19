use crate::LoxError;
use crate::expr::Expr;
use crate::scanning::{token::{Token, Value}, token_type::TokenType::{self, *}};
use crate::statement::Statement;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Builds ASTs for expressions
    pub fn parse(&mut self) -> Vec<Result<Statement, LoxError>> {
        let mut statements: Vec<Result<Statement, LoxError>> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        statements
    }

    fn statement(&mut self) -> Result<Statement, LoxError> {
        if self.match_token(&[Print]) {
            self.print_statement()
        }

        else if self.match_token(&[Var]) {
            match self.declaration() {
                Ok(stmt) => Ok(stmt),
                Err(error) => {
                    self.sync();
                    Err(error)
                }
            }
        }

        else {
            self.expression_statement()
        }
    }
    
    /// Consumes declarations.
    fn declaration(&mut self) -> Result<Statement, LoxError> {
        let name: Token = self.consume(Identifier, "Expect variable name.")?;
        let mut initializer: Option<Expr> = None;

        if self.match_token(&[Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(Semicolon, "Expect ';' after value.")?;

        Ok(Statement::Var(name, initializer))
    }

    /// Consumes print statements.
    fn print_statement(&mut self) -> Result<Statement, LoxError> {
        let value: Result<Expr, LoxError> = self.expression();
        self.consume(Semicolon, "Expect ';' after value.")?;
        
        match value {
            Ok(exp) => Ok(Statement::Print(exp)),
            Err(err) => Err(err)
        }
    }

    /// Consumes expressions.
    fn expression_statement(&mut self) -> Result<Statement, LoxError> {
        let expr: Result<Expr, LoxError> = self.expression();
        self.consume(Semicolon, "Expect ';' after value.")?;
        
        match expr {
            Ok(exp) => Ok(Statement::Expression(exp)),
            Err(err) => Err(err)
        }
    }

    /// Builds ASTs for expressions
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    /// Builds ASTs for equality
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut exp: Expr = self.comparison()?;

        while self.match_token(&[NotEqual, EqualEqual]) {
            let op: Token = self.previous();
            let right: Expr = self.comparison()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }

    /// Builds ASTs for comparisons
    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut exp: Expr = self.term()?;

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op: Token = self.previous();
            let right: Expr = self.term()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }

    /// Builds ASTs for arithmetic expressions
    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut exp: Expr = self.factor()?;

        while self.match_token(&[Minus, Plus]) {
            let op: Token = self.previous();
            let right: Expr = self.factor()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }

    /// Builds ASTs for geometric expressions
    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut exp: Expr = self.unary()?;

        while self.match_token(&[Slash, Asterisk]) {
            let op: Token = self.previous();
            let right: Expr = self.unary()?;
            exp = Expr::Binary(Box::new(exp), op, Box::new(right));
        }

        Ok(exp)
    }
    
    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&[Not, Minus]) {
            let op: Token = self.previous();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        match self.tokens[self.current].token_type {
            False => { 
                self.advance();
                Ok(Expr::Literal(Value::Bool(false)))
            },
            True => {
                self.advance();
                Ok(Expr::Literal(Value::Bool(true)))
            },
            Nil => {
                self.advance();
                Ok(Expr::Literal(Value::Nil()))
            },
            Int | Float | Str => {
                self.advance();
                Ok(Expr::Literal(self.previous().literal))
            },
            Identifier => {
                self.advance();
                Ok(Expr::Variable(self.previous().clone()))
            }
            LeftParen => {
                self.advance();
                let exp: Expr = self.equality()?;
                self.consume(RightParen, "Expect ')' after expression.")?;
                Ok(exp)
            },
            _ => Err(LoxError::ParseError(self.tokens[self.current].clone(), String::from("Invalid literal.")))
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
    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, LoxError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(LoxError::ParseError(self.peek(), msg.to_string()))
    }
    
    /// Resyncs the parser in the vent of an error
    fn sync(&mut self) {
        self.advance();

        while !self.is_at_end() {
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