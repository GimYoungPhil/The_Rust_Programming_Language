use std::collections::HashMap;

pub fn run() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Blue"), 200);

    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }
}
