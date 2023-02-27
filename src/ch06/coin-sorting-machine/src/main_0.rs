#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{} cents", value_in_cents(Coin::Penny));
    println!("{} cents", value_in_cents(Coin::Nickle));
    println!("{} cents", value_in_cents(Coin::Dime));
    println!("{} cents", value_in_cents(Coin::Quarter));
}
