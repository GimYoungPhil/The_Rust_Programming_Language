#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{}", value);
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
