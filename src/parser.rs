use crate::error::{parser_error, Error};
use crate::syntax::{Expr, LiteralValue, Stmt};
use crate::token::{Token, TokenType};

pub struct Parser<'t> {
    tokens: &'t Vec<Token>,
    current: usize,
}

/// bro i just copied this shit,
/// i will learn about
/// macros later i swear
macro_rules! matches {
    ( $sel:ident, $( $x:expr ),* ) => {
        {
            if $( $sel.check($x) )||* {
                $sel.advance();
                true
            } else {
                false
            }
        }
    };
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        token_type == self.peek().ttype
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Peek into da end of the token stream.")
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        let statement = if matches!(self, TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match statement {
            Err(Error::Parse) => {
                self.synchronize();
                Ok(Stmt::Null)
            }
            other => other,
        }
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if matches!(self, TokenType::Else) {
            Box::new(Some(self.statement()?))
        } else {
            Box::new(None)
        };

        Ok(Stmt::If {
            condition,
            else_branch,
            then_branch,
        })
    }

    fn or_(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and_()?;

        while matches!(self, TokenType::Or) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.and_()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and_(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while matches!(self, TokenType::And) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }


    fn statement(&mut self) -> Result<Stmt, Error> {
        if matches!(self, TokenType::For) {
            self.for_statement()
        } else if matches!(self, TokenType::If) {
            self.if_statement()
        } else if matches!(self, TokenType::Print) {
            self.print_statement()
        } else if matches!(self, TokenType::While) {
            self.while_statement()
        } else if matches!(self, TokenType::LeftBrace) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }
    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if matches!(self, TokenType::Semicolon) {
            None
        } else if matches!(self, TokenType::Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            let inc_stmt = Stmt::Expression { expression: inc };
            body = Stmt::Block {
                statements: vec![body, inc_stmt],
            }
        }

        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Literal {
                value: LiteralValue::Boolean(true),
            }),
            body: Box::new(body),
        };

        if let Some(init_stmt) = initializer {
            body = Stmt::Block {
                statements: vec![init_stmt, body],
            }
        }

        Ok(body)
    }
    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "uh I expected ';' (da semi colon) after value.",
        )?;
        Ok(Stmt::Print { expression: value })
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "expected variable name :/")?;
        let initializer = if matches!(self, TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "uh i expected ';' after variable declaration",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "uhh i expected ';' after expression :/",
        )?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "expect a '}' after block :/")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.or_()?;
        if matches!(self, TokenType::Equal) {
            let value = Box::new(self.assignment()?);
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign { name, value });
            }

            let equals = self.previous();
            self.error(equals, "ASSIGNMENT TARGET INVALID :| ");
        }
        Ok(expr)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
    fn unary(&mut self) -> Result<Expr, Error> {
        if matches!(self, TokenType::Bang, TokenType::Minus) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(ttype) {
            Ok(self.advance().clone())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        // We don't use matches!() here since we want to extract the literals.
        let expr = match &self.peek().ttype {
            TokenType::False => Expr::Literal {
                value: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                value: LiteralValue::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                value: LiteralValue::Null,
            },
            TokenType::String { literal } => Expr::Literal {
                value: LiteralValue::String(literal.clone()),
            },
            TokenType::Number { literal } => Expr::Literal {
                value: LiteralValue::Number(literal.clone()),
            },
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => return Err(self.error(self.peek(), "Expect expression.")),
        };

        self.advance();

        Ok(expr)
    }
    fn error(&self, token: &Token, message: &str) -> Error {
        parser_error(token, message);
        Error::Parse
    }

    fn multiplication(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while matches!(self, TokenType::Slash, TokenType::Star) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    fn addition(&mut self) -> Result<Expr, Error> {
        let mut expr = self.multiplication()?;

        while matches!(self, TokenType::Minus, TokenType::Plus) {
            let operator: Token = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Previous was empty.")
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.addition()?;
        while matches!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator: Token = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
}
