#[derive(Debug)]

enum ScotchKind {
    BlendedMalt,
    SingleMalt,
    Grain,
    Blended,
}

struct ScotchWhisky {
    kind: ScotchKind,
}

enum IrishKind {
    Blended,
    SingleMalt,
    SinglePotStill,
}

struct IrishWhisky {
    kind: IrishKind,
}

enum AmericanKind {
    Bourbon,
    Corn,
    Rye,
    Tennessee,
}

struct AmericanWhisky {
    kind: AmericanKind,
}

enum Whisky {
    Scotch(ScotchWhisky),
    Irish(IrishWhisky),
    American(AmericanWhisky),
    Canadian,
    Japanese,
}

// struct Whisky {
//     // area
//     kind: WhiskyKind,
//     // ml
//     volume: f32,
//     // %
//     alcohol: f32,
// }


fn main() {

    println!("{:?} {:?} {:?}", r, g, b);
}
