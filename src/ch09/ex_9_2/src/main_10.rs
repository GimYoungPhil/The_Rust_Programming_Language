use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let name = read_username_from_file().expect("파일을 열 수 없어요.");
    println!("{}", name);
}
