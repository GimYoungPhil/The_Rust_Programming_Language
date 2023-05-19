pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup(self, other: Point<T, U>) -> Point<T, U> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() -> () {
    let p0 = Point { x: 10, y: 10.1 };
    let p1 = Point { x: 20, y: 20.2 };

    let p2 = p0.mixup(p1);
    println!("p.x = {}, p.y = {}", p2.x, p2.y);
}

