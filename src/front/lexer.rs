use std::rc::Rc;

use super::{
    ast::{self, Expression, Program, Statement},
    token::{Token, TokenType},
};
use anyhow::{Ok, Result};
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

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>> {
        let expr = self.parse_expression()?;
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>> {}

    fn top(&mut self) -> Rc<Token> {
        let token = self.tokens[0].clone();
        token
    }
    fn eat(&mut self) -> Rc<Token> {
        self.tokens.remove(0)
    }
    fn is_eof(&self) -> bool {
        match self.top().t_type {
            TokenType::Eof => true,
            _ => false,
        }
    }
}
