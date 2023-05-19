pub struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn x(&self) -> f32 {
        self.x
    }
}

pub fn run() -> () {
    let p0 = Point { x: 5, y: 10 };
    let p1 = Point { x: 1.0, y: 4.0 };

    // println!("p.x = {}", p0.x());
    println!("p.x = {}", p1.x());
}

