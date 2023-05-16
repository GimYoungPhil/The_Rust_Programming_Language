fn main() {

    let x: i32 = 10;
    let y: Option<i32> = Some(20);

    println!("{}", x);
    println!("{:?}", y);

    let re = build_half(0);

    match re {
        Some(v) => println!("value: {}", v),
        None => println!("It's None!!"),
    }

}


fn build_half(x: i32) -> Option<i32> {
    let thousand: i32 = 1_000;

    if (x > 0) {
        return Some(x / thousand);
    } else {
        return None;
    }
}
