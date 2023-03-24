#[derive(Debug)]
struct Idol {
    weight: f32,
    height: f32,
}

impl Idol {
    fn bmi(&self) -> f32 {
        self.weight / self.height.powf(2.0)
    }

    fn plus_height(&mut self, val: f32) -> f32 {
        let new_height = self.height + val;
        self.height = new_height;

        new_height
    }
}

fn main() {
    let mut sakura: Idol = Idol {
        weight: 42.0,
        height: 1.63,
    };

    println!("Idol: {:#?}", sakura);
    println!("BMI: {:.1}", sakura.bmi());

    sakura.plus_height(0.01);
    println!("Idol: {:#?}", sakura);
    println!("BMI: {:.1}", sakura.bmi());
}
