fn main() -> () {

    let mut country = String::from("Austria");
    // let country_ref = &country;

    {
        let c = &country;
        println!("Go {}", c);
    }

    country.push('!');
    // println!("{}, {}", country_ref, country);

    println!("Back {}", country);
}

