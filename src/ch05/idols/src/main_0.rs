fn main() {
    // kg
    let weight: f32 = 42.0;
    // m
    let height: f32 = 1.63;

    println!("BMI: {:.1}", calculate_bmi(weight, height));
}

fn calculate_bmi(weight: f32, height: f32) -> f32 {
    weight / height.powf(2.0)
}
