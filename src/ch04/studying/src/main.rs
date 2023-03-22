use std::string;

fn main() {
    let s1: String = String::from("Hello Crustacea");

    let word: &str = first_word(&s1[..]);
    println!("first word: {}", word);

    let string_literal: &str = "hello world";
    let word: &str = first_word(&string_literal[..]);
    println!("first word: {}", word);

    let word: &str = first_word(string_literal);
    println!("first word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
