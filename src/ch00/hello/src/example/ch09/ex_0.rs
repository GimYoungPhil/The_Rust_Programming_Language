pub fn run() -> () {
    let numbers = vec![10, 2, 4, 25, 66, 99, 100, 90, 12];
    let num = largest(&numbers);
    println!("Largest: {}", num);

    let numbers = vec![444, 555, 666, 888, 999, 777];
    let num = largest(&numbers);
    println!("Largest: {}", num);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }

    largest
}
