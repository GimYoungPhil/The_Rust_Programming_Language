fn main() {
    struct Ract {
        width: u32,
        height: u32,
    }

    let r = Ract {
        width: 30,
        height: 50,
    };

    let area = r.width * r.height;

    println!("area: {}", area);
}

