use backyard::alcohol::whiskey::{tennessee};
use backyard::alcohol::{build_sake, build_hakaisan};

use garden::vegetables::Asparagus;
use crate::garden::fruit::{GrapeFruit, Watermelon};

pub mod garden;

fn main() {
    let plant0 = Asparagus {};
    println!("Hello, world! {:?}", plant0);

    let plant1 = GrapeFruit {};
    println!("Hello, world! {:?}", plant1);

    let plant2 = Watermelon {};
    println!("Hello, world! {:?}", plant2);

    let bottle0 = backyard::alcohol::whiskey::tennessee::JackDanial {};
    println!("Hello, world! {:?}", bottle0);

    let bottle1 = backyard::alcohol::whiskey::bourbon::JimBeam {};
    println!("Hello, world! {:?}", bottle1);

    let bottle2 = tennessee::JackDanial {};
    println!("Hello, world! {:?}", bottle2);

    let bottle3 = bourbon::JimBeam {};
    println!("Hello, world! {:?}", bottle3);

    let some = build_sake();
    println!("Hello, world! {:?}", some);

    let any = build_hakaisan();
    println!("Hello, world! {:?}", any);
}
