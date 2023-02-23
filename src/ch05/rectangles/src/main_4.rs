#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        println!("{} x {}", other.width, other.height);

        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 240,
        height: 40,
    };

    println!("{}", rect0.can_hold(&rect1));

    // println!("{:#?}", rect);
    // println!("area: {}", rect.area());
}
