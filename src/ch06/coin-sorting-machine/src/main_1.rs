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

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    println!("{} cents", value_in_cents(Coin::Penny));
    println!("{} cents", value_in_cents(Coin::Nickle));
    println!("{} cents", value_in_cents(Coin::Dime));
    println!("{} cents", value_in_cents(Coin::Quarter(UsState::Kentucky)));
    println!("{} cents", value_in_cents(Coin::Quarter(UsState::Pennsylvania)));
}
