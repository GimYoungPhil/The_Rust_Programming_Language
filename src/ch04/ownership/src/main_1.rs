fn main() {
    println!("Hello, world!");

    {
        let s = "Hello";

        println!("s: {}", s);
    }
    println!("s: {}", s);
}
