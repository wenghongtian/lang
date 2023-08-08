use anyhow::{anyhow, Result};
use std::rc::Rc;

use super::utils;

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Operator,
    OpenParen,
    CloseParen,
    Equal,
    Dot,

    NumberLiteral,

    Eof,
}

#[derive(Debug)]
pub enum TokenVal {
    Str(String),
    Number(f32),
}
#[derive(Debug)]

pub struct Token {
    pub t_type: TokenType,
    pub val: TokenVal,
}

impl Token {
    fn new(t_type: TokenType, val: TokenVal) -> Token {
        Token { t_type, val }
    }
}

fn token(t_type: TokenType, val: TokenVal) -> Rc<Token> {
    Rc::new(Token::new(t_type, val))
}

pub fn tokenlize(source: &str) -> Result<Vec<Rc<Token>>> {
    let mut src: Vec<&str> = source.split("").collect();
    let mut tokens: Vec<Rc<Token>> = vec![];

    while !src.is_empty() {
        let c = src.remove(0);

        match c {
            c if utils::is_op(c) => {
                tokens.push(token(TokenType::Operator, TokenVal::Str(c.to_string())))
            }

            "(" => tokens.push(token(TokenType::OpenParen, TokenVal::Str("(".to_string()))),

            ")" => tokens.push(token(TokenType::CloseParen, TokenVal::Str(")".to_string()))),

            c if utils::is_space(c) => (),

            c if utils::is_digit(c) => {
                let mut get_dot = false;
                let mut num_str = String::from(c);
                while !src.is_empty()
                    && (utils::is_digit(src.get(0).unwrap())
                        || *src.get(0).unwrap() == "." && !get_dot)
                {
                    let c = src.remove(0);
                    num_str.push_str(c);
                    if c == "." {
                        get_dot = true;
                    }
                }
                if get_dot && *src.get(0).unwrap() == "." {
                    return Err(anyhow!("Invalid number durint parsed."));
                }
                let num: f32 = num_str.parse().unwrap();
                tokens.push(token(TokenType::NumberLiteral, TokenVal::Number(num)));
            }
            c if utils::is_alpha(c) => {
                let mut id = String::from(c);
                while !src.is_empty()
                    && (utils::is_digit(src.get(0).unwrap())
                        || utils::is_alpha(src.get(0).unwrap()))
                {
                    id.push_str(src.remove(0));
                }
                tokens.push(token(TokenType::Identifier, TokenVal::Str(id)));
            }
            _ => return Err(anyhow!("Unrecongnized character during parsed.")),
        }
    }
    tokens.push(token(TokenType::Eof, TokenVal::Str("eof".to_string())));
    Ok(tokens)
}
