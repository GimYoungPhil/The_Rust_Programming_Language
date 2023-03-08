fn main() {
    let hello = String::from("안녕하세요");
    let answer = &hello[6..9];

    println!("{}", answer);
}
