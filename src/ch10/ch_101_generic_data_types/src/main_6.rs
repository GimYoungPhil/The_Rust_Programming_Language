fn main() {
    let mut number: i32 = 9999;

    let another_number = &mut number;

    // number = 888;
    // *another_number = 777;

    println!("number: {}", number);
    println!("another: {}", another_number);
}
