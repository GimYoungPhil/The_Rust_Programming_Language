mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // won't compile
    // meal.seasonal_fruit = String::from("Blueberries");
}
