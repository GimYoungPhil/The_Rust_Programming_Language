use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").expect("파일을 열 수 없어요.");
}
