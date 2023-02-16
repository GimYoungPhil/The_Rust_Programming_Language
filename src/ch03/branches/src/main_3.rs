fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number % 4 == 0");
    } else if number % 3 == 0 {
        println!("number % 3 == 0");
    } else if number % 2 == 0 {
        println!("number % 2 == 0");
    } else {
        println!("number % 1 == 0");
    }
}
