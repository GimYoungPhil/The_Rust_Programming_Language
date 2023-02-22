fn main() {
    let mut s = String::from("The RUST programming Language");

    let word = first_word(&s);

    s.clear();

    println!("s: {}", s);
    println!("w: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
