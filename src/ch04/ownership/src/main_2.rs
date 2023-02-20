fn main() {
    println!("Hello, world!");

    {
        let mut s = String::from("Hello");

        s.push_str(", world!");

        println!("s: {}", s);
    }
    // println!("s: {}", s);
}
