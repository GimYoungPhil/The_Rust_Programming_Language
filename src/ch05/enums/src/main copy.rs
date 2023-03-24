#[derive(Debug)]

enum ScotchKind {
    BlendedMalt,
    SingleMalt,
    Grain,
    Blended,
}

enum IrishKind {
    Blended,
    SingleMalt,
    SinglePotStill,
}

enum AmericanKind {
    Bourbon,
    Corn,
    Rye,
    Tennessee,
}

enum WhiskyKind {
    Scotch,
    Irish,
    American,
    Canadian,
    Japanese,
}

struct Whisky {
    // area
    kind: WhiskyKind,
    // ml
    volume: f32,
    // %
    alcohol: f32,
}


fn main() {

    println!("{:?} {:?} {:?}", r, g, b);
}
