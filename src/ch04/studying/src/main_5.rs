fn main() {
    let mut s1: String = String::from("Crustacea");

    greeting(&mut s1);

    println!("{}", s1);
}

fn greeting(someone: &mut String) {

    someone.insert_str(0, "Sir. ");

    println!("Hello {}!", someone);
}
