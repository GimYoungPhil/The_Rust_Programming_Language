fn main() {
    let rect = (30, 50);

    println!("area: {}", area(rect));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
