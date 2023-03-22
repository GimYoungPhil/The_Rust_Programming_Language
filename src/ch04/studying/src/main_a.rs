fn main() {
    let age_old_tom: u32 = 37;

    let age_young_tom = time_machine(age_old_tom);

    println!("Tom's Age: {}", age_young_tom);
}

fn time_machine(age_someone: u32) -> u32 {
    age_someone.saturating_sub(10)
}
