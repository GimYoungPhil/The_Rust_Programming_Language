#[derive(Debug)]

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let r = Color::Red;
    let g = Color::Green;
    let b = Color::Blue;
    println!("{:?} {:?} {:?}", r, g, b);
}
