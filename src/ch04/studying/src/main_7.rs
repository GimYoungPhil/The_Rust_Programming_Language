fn main() {
    let mut s1: String = String::from("Crustacea");
    println!("{}", s1);

    let s2 = &mut s1;
    println!("{}", s2);

    let s3 = &mut s1;
    println!("{}", s3);

    println!("{}", s1);
    println!("{}", s2);
}
