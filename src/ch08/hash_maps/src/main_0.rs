use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, f32> = HashMap::new();

    scores.insert("Mercury", 0.4);
    scores.insert("Venus", 0.7);
    scores.insert("Earth", 1.0);
    scores.insert("Mars", 1.5);
}
