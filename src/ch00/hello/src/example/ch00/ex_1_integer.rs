use std::mem::size_of;

pub fn run() -> () {

    let x0: i8 = 0;
    let x1: i8 = 1;
    let x2: i8 = 2;
    let x3: i8 = 3;
    let x4: i8 = 4;
    let x5: i8 = 5;

    let x6: i8 = -1;
    let x7: i8 = -2;
    let x8: i8 = -3;
    let x9: i8 = -4;
    let x10: i8 = -5;
    let x11: i8 = -6;

    let y0: u8 = 0;
    let y1: u8 = 1;
    let y2: u8 = 2;
    let y3: u8 = 3;
    let y4: u8 = 4;
    let y5: u8 = 5;

    println!("{:_^37}", "signed");
    println!("{:_^8}|_{:_^6}_|_{:_^18}", "Type", "Size", "Address");
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x0);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x1);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x2);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x3);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x4);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x5);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x6);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x7);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x8);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x9);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x10);
    println!("{: <8}| {: >6} | {:#p}", "i8", size_of::<i8>(), &x11);
    println!();

    println!("{:_^37}", "Unsigned");
    println!("{:_^8}|_{:_^6}_|_{:_^18}", "type", "size", "address");
    println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y0);
    println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y1);
    println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y2);
    println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y3);
    println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y4);
    println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y5);
    println!();

}
