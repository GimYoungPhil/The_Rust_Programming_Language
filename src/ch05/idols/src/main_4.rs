#[derive(Debug)]
struct Idol {
    weight: f32,
    height: f32,
}

impl Idol {
    fn bmi(&self) -> f32 {
        self.weight / self.height.powf(2.0)
    }
}

fn main() {
    let sakura: Idol = Idol {
        weight: 42.0,
        height: 1.63,
    };

    println!("Idol: {:#?}", sakura);
    println!("BMI: {:.1}", sakura.bmi());
}
