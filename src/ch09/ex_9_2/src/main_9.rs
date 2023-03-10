use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    let mut username_file = File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let name = read_username_from_file().expect("파일을 열 수 없어요.");
    println!("{}", name);
}
