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

    fn from(weight: f32, height: f32) -> Idol {
        Idol {
            weight,
            height,
        }
    }
}

fn main() {
    let sakura = Idol {
        weight: 42.0,
        height: 1.63,
    };

    let kazuha = Idol::from(47.0, 1.70);
    let sol = Idol::from(41.0, 1.59);

    println!("Sakura is over kazuha: {}", sakura.can_over(&kazuha));
    println!("Sakura is over sol: {}", sakura.can_over(&sol));
}
