struct Idol {
    weight: f32,
    height: f32,
}

fn main() {
    let sakura = Idol {
        weight: 42.0,
        height: 1.63,
    };

    println!("BMI: {:.1}", calculate_bmi(sakura));
}

fn calculate_bmi(idol: Idol) -> f32 {
    idol.weight / idol.height.powf(2.0)
}
