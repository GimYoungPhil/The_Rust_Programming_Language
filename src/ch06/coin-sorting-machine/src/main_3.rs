fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
    }
}

fn main() {
    println!("{:?}", plus_one(None));
    println!("{:?}", plus_one(Some(10)));
}
