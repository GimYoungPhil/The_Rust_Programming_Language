use std::mem::size_of;

fn main() -> () {

    let x0: i8 = 1;
    let x1: i16 = 2;
    let x2: i32 = 3;
    let x3: i64 = 4;
    let x4: i128 = 5;
    let x5: isize = 6;

    let y0: u8 = 7;
    let y1: u16 = 8;
    let y2: u32 = 9;
    let y3: u64 = 10;
    let y4: u128 = 11;
    let y5: usize = 12;

    println!("{:_^30}", "signed");
    println!("{:<8} | {:>4} | {:p}", "i8",    size_of::<i8>(),    &x0);
    println!("{:<8} | {:>4} | {:p}", "i16",   size_of::<i16>(),   &x1);
    println!("{:<8} | {:>4} | {:p}", "i32",   size_of::<i32>(),   &x2);
    println!("{:<8} | {:>4} | {:p}", "i64",   size_of::<i64>(),   &x3);
    println!("{:<8} | {:>4} | {:p}", "i128",  size_of::<i128>(),  &x4);
    println!("{:<8} | {:>4} | {:p}", "isize", size_of::<isize>(), &x5);

    println!("{:_^30}", "unsigned");
    println!("{:<8} | {:>4} | {:p}", "u8",    size_of::<u8>(),    &y0);
    println!("{:<8} | {:>4} | {:p}", "u16",   size_of::<u16>(),   &y1);
    println!("{:<8} | {:>4} | {:p}", "u32",   size_of::<u32>(),   &y2);
    println!("{:<8} | {:>4} | {:p}", "u64",   size_of::<u64>(),   &y3);
    println!("{:<8} | {:>4} | {:p}", "u128",  size_of::<u128>(),  &y4);
    println!("{:<8} | {:>4} | {:p}", "usize", size_of::<usize>(), &y5);
    println!("{:_^30}", "end");
}
