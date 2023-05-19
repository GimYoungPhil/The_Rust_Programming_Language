pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() -> () {
    let p0 = Point { x: 10, y: 10.1 };
    let p1 = Point { x: 20_u32, y: 30_u64 };

    let p2 = p0.mixup(p1);
    println!("p.x = {}, p.y = {}", p2.x, p2.y);
}

