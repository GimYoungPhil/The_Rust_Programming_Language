#![allow(unused)]

#[derive(Debug)]
enum UsState {
    // 1999
    Delaware,
    Pennsylvania,
    NewJersey,
    Georgia,
    Connecticut,
    // 2000
    Massachusetts,
    Maryland,
    SouthCarolina,
    NewHampshire,
    Virginia,
    // 2001
    NewYork,
    NorthCarolina,
    RhodeIsland,
    Vermont,
    Kentucky,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;

    match coin {
        Coin::Penny => println!("Penny"),
        _ => (),
    }

    if let Coin::Penny = coin {
        println!("Penny");
    }
}
