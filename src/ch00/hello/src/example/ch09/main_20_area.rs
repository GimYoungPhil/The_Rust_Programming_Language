struct Rectangle(u32, u32);
fn main() {

    let rect = Rectangle(30, 50);

    println!("Rectangle: {}", area(rect));
}

fn area(r: Rectangle) -> u32 {
    r.0 * r.1
}
