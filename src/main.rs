mod front;

fn main() {
    // let tokens = front::tokenlize("1+2.1").unwrap();
    // println!("{:#?}", tokens);
    let mut parser = front::lexer::Parser::new();
    let program = parser.parse("1 + 1").unwrap();
}
