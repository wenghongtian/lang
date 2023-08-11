use std::rc::Rc;

use super::{
    ast::{self, BinaryExpression, Expression, NumberLiteral, Program, Statement},
    token::{Token, TokenType, TokenVal},
};
use anyhow::{anyhow, Ok, Result};
pub struct Parser {
    tokens: Vec<Rc<Token>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { tokens: vec![] }
    }

    pub fn parse(&mut self, source: &str) -> Result<Program> {
        self.tokens = super::token::tokenlize(source)?;
        let mut program = ast::Program::new();
        while self.not_eof() {
            program.body.push(self.parse_statement()?)
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>> {
        let expr = self.parse_expression()?;
        unsafe {
            let ans = std::mem::transmute::<Box<dyn Expression>, Box<dyn Statement>>(expr);
            Ok(ans)
        }
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>> {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Result<Box<dyn Expression>> {
        let left = self.parse_multiplicitave_expression()?;
        if let TokenVal::Str(op) = &self.top().val {
            match op.as_str() {
                "+" | "-" => {
                    self.eat();
                    let operator = op.as_str();
                    let right = self.parse_multiplicitave_expression()?;
                    let left = BinaryExpression::new(left, right, operator.to_string());
                    return Ok(Box::new(left));
                }
                _ => (),
            }
        }
        Ok(left)
    }

    fn parse_multiplicitave_expression(&mut self) -> Result<Box<dyn Expression>> {
        let left = self.parse_primary_expression()?;
        if let TokenVal::Str(op) = &self.top().val {
            match op.as_str() {
                "*" | "/" | "%" => {
                    self.eat();
                    let operator = op.as_str();
                    let right = self.parse_expression()?;
                    let left = BinaryExpression::new(left, right, operator.to_string());
                    return Ok(Box::new(left));
                }
                _ => (),
            }
        }
        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> Result<Box<dyn Expression>> {
        match self.top().t_type {
            // TokenType::Identifier => todo!(),
            // TokenType::Operator => todo!(),
            TokenType::OpenParen => {
                self.eat();
                let expr = self.parse_expression();
                _ = self.expected(
                    TokenType::CloseParen,
                    "Expected close paren to match open paren!",
                );
                return expr;
            }
            // TokenType::Dot => todo!(),
            TokenType::NumberLiteral => {
                let token = self.eat();
                let val = &token.val;
                match val {
                    TokenVal::Number(n) => Ok(Box::new(NumberLiteral::new(*n))),
                    _ => Err(anyhow!("Unexpected val from token: {:?}", token)),
                }
            }
            _ => Err(anyhow!(
                "Unexpected token found during parsing! {:?}",
                self.top()
            )),
        }
    }

    fn top(&mut self) -> Rc<Token> {
        let token = self.tokens[0].clone();
        token
    }
    fn eat(&mut self) -> Rc<Token> {
        self.tokens.remove(0)
    }
    fn not_eof(&mut self) -> bool {
        match self.top().t_type {
            TokenType::Eof => false,
            _ => true,
        }
    }
    fn expected(&mut self, token_type: TokenType, err_msg: &str) -> Result<Rc<Token>> {
        if token_type == self.top().t_type {
            Ok(self.eat())
        } else {
            Err(anyhow!("{}", err_msg))
        }
    }
}
