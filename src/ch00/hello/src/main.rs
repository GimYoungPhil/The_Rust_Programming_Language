use std::mem::{size_of, size_of_val};

fn main() -> () {

    let x0: u8 = 0;
    let x1: u16 = 0;
    let x2: u32 = 0;
    let x3: u64 = 0;
    let x4: u128 = 0;
    let x5: usize = 0;

    let r0: &u8 = &x0;
    let r1: &u16 = &x1;
    let r2: &u32 = &x2;
    let r3: &u64 = &x3;
    let r4: &u128 = &x4;
    let r5: &usize = &x5;

    println!("{:_^22}", "unsigned");
    println!("{:<12} | {:>3} | {:>3}", "u8",    size_of::<u8>(),    size_of_val(&x0));
    println!("{:<12} | {:>3} | {:>3}", "u16",   size_of::<u16>(),   size_of_val(&x1));
    println!("{:<12} | {:>3} | {:>3}", "u32",   size_of::<u32>(),   size_of_val(&x2));
    println!("{:<12} | {:>3} | {:>3}", "u64",   size_of::<u64>(),   size_of_val(&x3));
    println!("{:<12} | {:>3} | {:>3}", "u128",  size_of::<u128>(),  size_of_val(&x4));
    println!("{:<12} | {:>3} | {:>3}", "usize", size_of::<usize>(), size_of_val(&x5));

    println!("{:_^22}", "reference");
    println!("{:<12} | {:>3} | {:>3}", "&u8",    size_of::<&u8>(),    size_of_val(r0));
    println!("{:<12} | {:>3} | {:>3}", "&u16",   size_of::<&u16>(),   size_of_val(r1));
    println!("{:<12} | {:>3} | {:>3}", "&u32",   size_of::<&u32>(),   size_of_val(r2));
    println!("{:<12} | {:>3} | {:>3}", "&u64",   size_of::<&u64>(),   size_of_val(r3));
    println!("{:<12} | {:>3} | {:>3}", "&u128",  size_of::<&u128>(),  size_of_val(r4));
    println!("{:<12} | {:>3} | {:>3}", "&usize", size_of::<&usize>(), size_of_val(r5));



}