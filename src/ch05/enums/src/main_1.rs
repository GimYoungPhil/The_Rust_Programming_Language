#[derive(Debug)]

enum AreaKind {
    Scotch,
    Irish,
    American,
    Canadian,
    Japanese,
    Taiwanese,
}

enum WhiskyKind {
    // Scotch
    BlendedMalt,
    SingleMalt,
    Grain,
    Blended,
    // American
    Bourbon,
    Corn,
    Rye,
    Tennessee,
    //
    Etc,
}

struct Whisky {
    // area
    area: AreaKind,
    //
    kind: WhiskyKind,
    // ml
    volume: f32,
    // %
    alcohol: f32,
}


fn main() {

    println!("{:?} {:?} {:?}", r, g, b);
}
