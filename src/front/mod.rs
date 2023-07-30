mod utils;

pub enum TokenType {
    Identifier,
    Operator,
    OpenParen,
    CloseParen,
    Equal,
    Dot,

    NumberLiteral,
}

enum TokenVal {
    Str(String),
    Number(f32),
}

struct Token {
    t_type: TokenType,
    val: TokenVal,
}

impl Token {
    fn new(t_type: TokenType, val: TokenVal) -> Token {
        Token { t_type, val }
    }
}

fn tokenlize(source: &str) -> Result<Vec<Token>, &str> {
    let mut src: Vec<&str> = source.split("").collect();
    let mut tokens: Vec<Token> = vec![];

    while !src.is_empty() {
        let c = src.remove(0);

        match c {
            c if utils::is_op(c) => tokens.push(Token::new(
                TokenType::Operator,
                TokenVal::Str(c.to_string()),
            )),
            "(" => tokens.push(Token {
                t_type: TokenType::OpenParen,
                val: TokenVal::Str("(".to_string()),
            }),
            ")" => tokens.push(Token {
                t_type: TokenType::CloseParen,
                val: TokenVal::Str("(".to_string()),
            }),
            num if utils::is_digit(c) => {
                let mut get_dot = false;
                let mut num_str = String::from(num);
                while !src.is_empty() && utils::is_digit(src.get(0).unwrap()) {
                    num_str.push_str(src.remove(0));
                    let next_c = src.get(0).unwrap();
                    if *next_c == "." {
                        src.remove(0);
                        if !get_dot {
                            get_dot = true;
                            num_str.push_str(".");
                        } else {
                            return Err("Invalid number");
                        }
                    }
                }
                let num: f32 = num_str.parse().unwrap();
                tokens.push(Token::new(TokenType::NumberLiteral, TokenVal::Number(num)));
            }
            c if utils::is_alpha(c) => {
                let mut id = String::from(c);
                while !src.is_empty()
                    && (utils::is_digit(src.get(0).unwrap())
                        || utils::is_alpha(src.get(0).unwrap()))
                {
                    id.push_str(src.remove(0));
                }
                tokens.push(Token::new(TokenType::Identifier, TokenVal::Str(id)));
            }
            _ => {}
        }
    }
    Ok(tokens)
}
