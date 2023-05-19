use std::collections::HashMap;

pub fn run() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }
}
