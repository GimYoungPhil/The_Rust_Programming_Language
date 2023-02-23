#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect);
    println!("area: {}", area(&rect));
}

fn area(reacangle: &Rectangle) -> u32 {
    reacangle.width * reacangle.height
}

