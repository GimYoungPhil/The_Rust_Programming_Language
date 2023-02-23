#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect0 = Rectangle::square(90);
    let rect1 = Rectangle::square(70);

    println!("{}", rect0.can_hold(&rect1));

    // println!("{:#?}", rect);
    // println!("area: {}", rect.area());
}
