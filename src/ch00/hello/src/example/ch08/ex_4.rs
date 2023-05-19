use std::collections::HashMap;

pub fn run() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    let score = scores.entry(String::from("Red")).or_insert(100);
    *score += 1;

    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }
}
