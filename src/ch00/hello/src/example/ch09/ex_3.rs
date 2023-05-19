struct Point<T, U> {
    x: T,
    y: U,
}

pub fn run() -> () {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 10, y: 4.0 };
}

