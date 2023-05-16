#[derive(Debug)]
struct Movie {
    title: String,
    year: u16,
    director: String,
}


fn main() {

    let a0 = Movie {
        title: "Alien".to_string(),
        year: 1979,
        director: "Ridley Scott".to_string(),
    };
    let a1 = Movie {
        title: "Aliens".to_string(),
        year: 1986,
        director: "James Cameron".to_string(),
    };
    let a2 = Movie {
        title: "Alien".to_string(),
        year: 1992,
        director: "David Fincher".to_string(),
    };
    let a3 = Movie {
        title: "Alien: Resurrection".to_string(),
        year: 1997,
        director: "Jean-Pierre Jeunet".to_string(),
    };
    let a4 = Movie {
        title: "Prometheus".to_string(),
        year: 2012,
        director: "Ridley Scott".to_string(),
    };
    let a5 = Movie {
        title: "Alien: Covenant".to_string(),
        year: 2017,
        director: "Ridley Scott".to_string(),
    };

    println!("Rectangle can hold");
}
