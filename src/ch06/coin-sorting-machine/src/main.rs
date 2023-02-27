fn plus_one(some_u8_value: u8) {
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn main() {
    plus_one(0_u8);
    plus_one(1_u8);
    plus_one(2_u8);
    plus_one(3_u8);
    plus_one(4_u8);
    plus_one(5_u8);
}
