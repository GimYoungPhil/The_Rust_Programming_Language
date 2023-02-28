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
    let mut count = 0;
    let coin = Coin::Quarter(UsState::NewYork);

    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
