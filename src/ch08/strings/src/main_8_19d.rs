fn main() {
    let hello = String::from("안녕하세요");
    let answer = &hello[0..1];

    println!("{}", answer); // panic
}
