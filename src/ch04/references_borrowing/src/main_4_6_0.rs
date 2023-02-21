fn main() {
    let s = String::from("hello");

    change(&s);

    println!("s: {}", s);
}

fn change(some_string: &String) {
    some_string.push_str(", world!");
}
