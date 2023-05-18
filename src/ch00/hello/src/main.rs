use example::ch00;
use example::ch01;
// use example::ch02;

// use hello::eat_at_restaurant;

use breakfast::madrid::seafood::Meal;
pub mod example;
pub mod breakfast;

fn main() {
    ch00::ex_0_integer::run();
    ch01::ex_01_array::run();

    let meal: Meal = Meal::summer("Shrimp");
    println!("{:?}", meal);
}


