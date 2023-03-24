fn main() {

    let sakura: (f32, f32) = (42.0, 1.63);

    println!("BMI: {:.1}", calculate_bmi(sakura));
}

fn calculate_bmi(physical: (f32, f32)) -> f32 {
    physical.0 / physical.1.powf(2.0)
}
