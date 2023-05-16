#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn add_width(&mut self, value: u32) {
        self.width += value;
    }
}

fn main() {

    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle: {}", rect.area());

    rect.add_width(10);
    println!("Rectangle: {}", rect.area());
}
