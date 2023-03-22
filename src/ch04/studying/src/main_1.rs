fn main() {
    let s1: String = String::from("Crustacea");

    let s2: String = s1;

    // error
    println!("{}", s1);

    println!("{}", s2);
}
