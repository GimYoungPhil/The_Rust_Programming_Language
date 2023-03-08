use std::collections::HashMap;

fn main() {
    let team = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = team.iter().zip(initial_scores.iter()).collect();

    for (s, i) in &scores {
        println!("{}: {}", s, i);
    }
}
