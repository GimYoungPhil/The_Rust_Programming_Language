fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("s: {}", s);
    println!("x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}
