fn main() {

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("len: {}", len);
}

fn calculate_length(s: String) -> (String, usize) {

    let length = s.len();

    (s, length)
}
