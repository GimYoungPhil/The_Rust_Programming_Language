fn main() {
    let months = [
        "Jaunary", "February", "March",
        "April",   "May",      "June",
        "July",    "August",   "September",
        "October", "November", "December",
    ];
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("months[0]: {}", months[0]);
    println!("a[0]: {}", a[0]);
    println!("b[0]: {}", b[0]);
    println!("c[0]: {}", c[0]);
}
