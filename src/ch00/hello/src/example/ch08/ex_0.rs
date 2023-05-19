use std::collections::HashMap;

pub fn run() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    let team: String = String::from("블루");
    let score: Option<&i32> = scores.get(&team);

    if let Some(v) = score {
      println!("{}", *v);
    }
}
