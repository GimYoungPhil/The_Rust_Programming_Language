fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("number: {}", number);
}
