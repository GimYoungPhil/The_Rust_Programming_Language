fn main() {
    let s1: String = String::from("Hello Crustacea");

    let word: &str = first_word(&s1);
    println!("first word: {}", word);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
