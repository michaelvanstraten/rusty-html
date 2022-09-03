use std::borrow::Cow;

fn main() {
    println!("Hello, {}!", "world")
}

fn test_const(inout: Vec<String>) -> Cow<String> {
    Cow::Owned("asdad".to_string())
}