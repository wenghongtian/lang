pub fn is_digit(c: &str) -> bool {
    c.parse::<i32>().is_ok()
}

pub fn is_alpha(c: &str) -> bool {
    c.chars().all(|ic| ic.is_ascii_alphabetic())
}

pub fn is_op(c: &str) -> bool {
    match c {
        "+" | "-" | "*" | "/" | "%" | "&" | "|" => true,
        _ => false,
    }
}
