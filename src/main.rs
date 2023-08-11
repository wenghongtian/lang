use anyhow::{Result, Ok};
use dialoguer::{console::style, Input};
use runtime::eval;

mod front;
mod runtime;

fn main() -> Result<()> {
    let mut parser = front::lexer::Parser::new();
    println!("{} v1.0:", style("r-lang").cyan());
    loop {
        let input: String = Input::new().with_prompt(">").interact_text()?;
        if input == "exit" {
            break;
        }
        let ast = parser.parse(input.as_str()).unwrap();
        let val = eval(&ast);
        println!("{:#?}", style(val).cyan());
    }
    Ok(())
}
