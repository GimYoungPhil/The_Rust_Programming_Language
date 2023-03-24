#[derive(Debug)]
struct Idol {
    weight: f32,
    height: f32,
}

impl Idol {
    fn bmi(&self) -> f32 {
        self.weight / self.height.powf(2.0)
    }

    fn can_over(&self, other: &Idol) -> bool {
        self.weight > other.weight && self.height > other.height 
    }
}

fn main() {
    let sakura: Idol = Idol {
        weight: 42.0,
        height: 1.63,
    };

    let kazuha: Idol = Idol {
        weight: 47.0,
        height: 1.70,
    };

    let sol: Idol = Idol {
        weight: 41.0,
        height: 1.59,
    };

    println!("Sakura is over kazuha: {}", sakura.can_over(&kazuha));
    println!("Sakura is over sol: {}", sakura.can_over(&sol));
}
