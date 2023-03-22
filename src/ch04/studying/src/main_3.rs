fn main() {
    let s1: String = String::from("Crustacea");

    let s2: String = greeting(s1);

    println!("{}", s2);
}

fn greeting(someone: String) -> String {
    println!("Hello {}!", someone);

    someone
}
