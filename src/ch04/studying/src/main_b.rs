fn main() {
    let s = "RUST";

    greeting(s);
}

fn greeting(something: String) {
    println!("hello, {}", something)
}
