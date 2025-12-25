use crate::LoxError;
use crate::types::{expr::Expr, token::Token, values::Value, token_type::TokenType::{self, *}, statement::Statement};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Builds all ASTs needed for a program
    pub fn parse(&mut self) -> Vec<Result<Statement, LoxError>> {
        let mut statements: Vec<Result<Statement, LoxError>> = Vec::new();

        while !self.is_at_end() {
            let statement_result = self.statement();

            // We love band-aid fixes, don't we folks
            if let Err(_) = statement_result {
                self.advance();
            }

            statements.push(statement_result);
        }

        statements
    }

    /// Builds ASTs for statements
    fn statement(&mut self) -> Result<Statement, LoxError> {
        if self.match_token(&[Class]) { self.class_declaration() }
        else if self.match_token(&[Fun]) { self.function_declaration("function") }
        else if self.match_token(&[Var]) {
            match self.declaration() {
                Ok(stmt) => Ok(stmt),
                Err(error) => {
                    self.sync();
                    Err(error)
                }
            }
        }
        else if self.match_token(&[For]) { self.for_statement() }
        else if self.match_token(&[If]) { self.if_statement() }
        else if self.match_token(&[Print]) { self.print_statement() }
        else if self.match_token(&[Return]) { self.return_statement() }
        else if self.match_token(&[While]) { self.while_statement() }
        else if self.match_token(&[LeftBrace]) { Ok(Statement::Block(self.block()?)) }
        else { self.expression_statement() }
    }

    /// Consumes print statements.
    fn if_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume(LeftParen, "Expect '(' after 'if'.")?;
        let condition: Expr = self.expression()?;
        self.consume(RightParen, "Expect ')' after if condition.")?;

        let then_branch: Statement = self.statement()?;
        let mut else_branch: Option<Statement> = None;

        if self.match_token(&[Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Statement::If(condition, Box::new(then_branch), Box::new(else_branch)))
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

    /// Consumes blocks.
    fn block(&mut self) -> Result<Box<Vec<Statement>>, LoxError> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.check(&RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(RightBrace, "Expect '}' after block.")?;

        Ok(Box::new(statements))
    }

    /// Consumes while statements.
    fn while_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume(LeftParen, "Expect '(' after 'while'.")?;
        let condition: Expr = self.expression()?;
        self.consume(RightParen, "Expect '(' after 'while'.")?;
        let body: Statement = self.statement()?;

        Ok(Statement::While(condition, Box::new(body)))
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

    /// Consumes for statements.
    fn for_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume(LeftParen, "Expect '(' after 'for'.")?;

        // Build initializer
        let mut initializer: Option<Statement> = None;

        if self.match_token(&[Var]) {
            initializer = Some(self.declaration()?);
        }
        else {
            if !self.match_token(&[Semicolon]) {
                initializer = Some(self.expression_statement()?);
            }
        }

        // Build condition
        let mut condition: Expr = Expr::Literal(Value::Bool(true));

        if !self.check(&Semicolon) {
            condition = self.expression()?;
        }
        
        self.consume(Semicolon, "Expect ';' after loop condition.")?;

        // Build increment
        let mut increment: Option<Expr> = None;

        if !self.check(&RightParen) {
            increment = Some(self.expression()?);
        }
        
        self.consume(RightParen, "Expect ')' after for clauses.")?;

        // Build body
        let mut body: Statement = self.statement()?;

        if let Some(expr) = increment {
            body = Statement::Block(Box::new(vec![body, Statement::Expression(expr)]));
        };

        body = Statement::While(condition, Box::new(body));

        if let Some(expr) = initializer {
            body = Statement::Block(Box::new(vec![expr, body]));
        };

        Ok(body)
    }

    /// Consumes functions.
    fn function_declaration(&mut self, kind: &str) -> Result<Statement, LoxError> {
        let name: Token = self.consume(Identifier, format!("Expect {kind} name.").as_str())?;
        self.consume(LeftParen, format!("Expect '(' after {kind} name.").as_str())?;
        let mut params: Vec<Token> = Vec::new();

        // Consume params.
        if !self.check(&RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(LoxError::ArgumentError(Statement::Expression(
                                                        Expr::Literal(
                                                        Value::Identifier(
                                                        Box::new(self.peek().clone())
                                                       ))),
                                                       String::from("Can't have more than 255 parameters.")));
                }
                
                params.push(self.consume(Identifier, "Expect parameter name.")?);

                if !self.match_token(&[Comma]) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Expect ')' after parameters.")?;

        // Consume body.
        self.consume(LeftBrace, format!("Expect '{{' before {kind} body.").as_str())?;
        let body: Box<Vec<Statement>> = self.block()?;

        Ok(Statement::FunDeclaration(name, params, body))
    }

    // Consume return statements.
    fn return_statement(&mut self) -> Result<Statement, LoxError> {
        let mut value: Expr = Expr::Literal(Value::Nil());

        // Interpret return value, if it exists.
        if !self.check(&Semicolon) {
            value = self.expression()?;
        }

        self.consume(Semicolon, "Expect ';' after return value.")?;
        Ok(Statement::Return(value))
    }

    // Consumes class declarations.
    fn class_declaration(&mut self) -> Result<Statement, LoxError> {
        // Get names
        let name: Token = self.consume(Identifier, "Expect class name.")?;
        self.consume(LeftBrace, "Expect '{' before class body.")?;

        // Get body
        let mut methods: Vec<Statement> = Vec::new();

        while !self.check(&RightBrace) && !self.is_at_end() {
            methods.push(self.function_declaration("method")?);
        }

        self.consume(RightBrace, "Expect '}' after class body.")?;

        Ok(Statement::Class(name, Box::new(methods)))
    }

    /// Builds ASTs for expressions
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    /// Builds ASTs for assignment
    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let exp: Expr = self.or()?;

        // Consume equals sign
        if self.match_token(&[Equal]) {
            let equals: Token = self.previous();
            // Parse the expression's l-value recursively
            let value: Expr = self.assignment()?;

            if let Expr::Variable(name) = exp {
                return Ok(Expr::Assign(name, Box::from(value)))
            }

            else {
                return Err(LoxError::ParseError(equals, String::from("Invalid assignment target.")));
            }
        }

        Ok(exp)
    }

    /// Builds ASTs for logical ors
    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut exp: Expr = self.and()?;

        while self.match_token(&[Or]) {
            let operator: Token = self.previous();
            let right: Expr = self.and()?;
            exp = Expr::Logical(Box::new(exp), operator, Box::new(right))
        }

        Ok(exp)
    }

    /// Builds ASTs for logical ands
    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut exp: Expr = self.equality()?;

        while self.match_token(&[And]) {
            let operator: Token = self.previous();
            let right: Expr = self.equality()?;
            exp = Expr::Logical(Box::new(exp), operator, Box::new(right))
        }

        Ok(exp)
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
    
    /// Builds ASTs for arithmetic expressions
    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&[Not, Minus]) {
            let op: Token = self.previous();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }

        self.call()
    }

    /// Builds ASTs for function calls
    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.primary()?;

        loop {
            if self.match_token(&[LeftParen]) {
                expr = self.parse_arguments(expr)?;
            }

            else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parses function arguments.
    fn parse_arguments(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(&RightParen) {
            loop {
                arguments.push(self.expression()?);

                if arguments.len() > 255 {
                    return Err(LoxError::ArgumentError(Statement::Expression(callee), 
                                                       String::from("Can't have more than 255 arguments.")))
                }

                if !self.match_token(&[Comma]) {
                    break;
                }
            }
        }

        self.consume(RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call(Box::new(callee), Box::new(arguments)))
    }

    /// Parses literals.
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
                Ok(Expr::Variable(self.previous()))
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
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
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

        Err(LoxError::ParseError(self.peek().clone(), msg.to_string()))
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