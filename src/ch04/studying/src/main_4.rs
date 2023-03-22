fn main() {
    let s1: String = String::from("Crustacea");

    greeting(&s1);

    println!("{}", s1);
}

fn greeting(someone: &String) {
    println!("Hello {}!", someone);
}
