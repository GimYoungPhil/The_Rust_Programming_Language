fn main() {
    let v = vec![10, 20, 30, 40, 50];

    let third: &i32 = &v[2];
    println!("the third element is {}", third);

    let third: Option<&i32> = v.get(4);
    match third {
        Some(el) => println!("The third element is {el}"),
        None => println!("There is no third element")
    }
}
