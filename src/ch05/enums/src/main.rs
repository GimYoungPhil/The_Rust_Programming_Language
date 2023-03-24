#[derive(Debug)]
enum NumberKind {
    Ace,
    Num(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
enum SuitKind {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug)]
struct PlayingCard {
    suit: SuitKind,
    num: NumberKind,
}

fn main() {

    let deck: PlayingCard = PlayingCard {
        suit: SuitKind::Spade,
        num: NumberKind::Num(10),
    };

    println!("{:?}", deck);

}
