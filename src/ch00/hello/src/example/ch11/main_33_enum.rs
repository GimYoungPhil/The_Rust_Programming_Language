#[warn(dead_code)]

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
    let value = value_in_cents(coin);
    println!("{}", value);

    let my_coin = Coin::Quarter(UsState::Massachusetts);
    let my_value = value_in_cents(my_coin);
    println!("{}", my_value);
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::NewJersey => println!("New Jersey!!"),
                _ => println!("State quarter from {:?}", state),
            }
            25
        }
    }
}
