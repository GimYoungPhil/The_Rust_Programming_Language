fn main() {
    let x = 5;
    let y = (let x = 5);

    println!("x: {}", x);
    println!("y: {}", y);
}
