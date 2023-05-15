use std::mem::size_of;

fn main() -> () {

    let x0: i16 = 0;
    let x1: i16 = 1;
    let x2: i16 = 2;
    let x3: i16 = 3;
    let x4: i16 = 4;
    let x5: i16 = 5;

    let x6: i16 = -1;
    let x7: i16 = -2;
    let x8: i16 = -3;
    let x9: i16 = -4;


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
    println!();

    // println!("{:_^37}", "Unsigned");
    // println!("{:_^8}|_{:_^6}_|_{:_^18}", "type", "size", "address");
    // println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y0);
    // println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y1);
    // println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y2);
    // println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y3);
    // println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y4);
    // println!("{: <8}| {: >6} | {:#p}", "u8", size_of::<u8>(), &y5);
    // println!();

}
