fn main() {
    // Vec<i32>
    let mut v = Vec::new();
    println!("{}", v.len());

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{}", v.len());
}
