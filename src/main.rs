mod front;

fn main() {
    let tokens = front::tokenlize("1+2.1").unwrap();
    println!("{:#?}", tokens);
}
