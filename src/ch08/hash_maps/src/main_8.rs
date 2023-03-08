use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 100);

    println!("{:?}", scores);
}
