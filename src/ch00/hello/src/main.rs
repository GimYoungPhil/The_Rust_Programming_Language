fn main() {
    let some_u8_value = Some(3u8);

    match some_u8_value {
        Some(v) => println!("Some {}", v),
        _ => (),
    }

    if let Some(v) = some_u8_value {
        println!("Some {}", v);
    } else {
        println!("None...");
    }

    if let Some(3) = some_u8_value {
        println!("Three");
    }


}


