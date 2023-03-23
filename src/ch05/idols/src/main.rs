fn main() {
    let weight: i32 = 42;
    let height: i32 = 163;

    println!("BMI: {}", calculate_bmi(weight as f32, height as f32));
}

fn calculate_bmi(weight: f32, height: f32) -> f32 {
    weight / (height / 100.0).powf(2.0)
}
