#[derive(Debug)]
struct Idol {
    weight: f32,
    height: f32,
}

fn main() {
    let sakura: Idol = Idol {
        weight: 42.0,
        height: 1.63,
    };

    println!("Idol: {:#?}", sakura);
}
