fn main() {
    let s1: String = String::from("hello");
    let s2: &str = "RUST";

    greeting_string(s1);
    greeting_str(s2);
}

fn greeting_string(something: String) {
    println!("String: {}, world", something);
}

fn greeting_str(something: &str) {
    println!("&str: {}, world", something);
}
