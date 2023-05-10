use std::mem::{size_of, size_of_val};

fn main() -> () {

    // let x0: u8 = 0;
    // let x1: u16 = 0;
    // let x2: u32 = 0;
    // let x3: u64 = 0;
    // let x4: u128 = 0;
    // let x5: usize = 0;

    let a0: [u8; 5] = [0; 5];
    let a1: [u16; 5] = [0; 5];
    let a2: [u32; 5] = [0; 5];
    let a3: [u64; 5] = [0; 5];
    let a4: [u128; 5] = [0; 5];
    let a5: [usize; 5] = [0; 5];

    let p0: &[u8; 5] = &a0;
    let p1: &[u16; 5] = &a1;
    let p2: &[u32; 5] = &a2;
    let p3: &[u64; 5] = &a3;
    let p4: &[u128; 5] = &a4;
    let p5: &[usize; 5] = &a5;

    let s0: &[u8] = &a0[..];
    let s1: &[u16] = &a1[..];
    let s2: &[u32] = &a2[..];
    let s3: &[u64] = &a3[..];
    let s4: &[u128] = &a4[..];
    let s5: &[usize] = &a5[..];

    // let v0: Vec<u8> = vec![0; 5];
    // let v1: Vec<u16> = vec![0; 5];
    // let v2: Vec<u32> = vec![0; 5];
    // let v3: Vec<u64> = vec![0; 5];
    // let v4: Vec<u128> = vec![0; 5];
    // let v5: Vec<usize> = vec![0; 5];

    // let r0: &[u8] = &v0[..];
    // let r1: &[u16] = &v1[..];
    // let r2: &[u32] = &v2[..];
    // let r3: &[u64] = &v3[..];
    // let r4: &[u128] = &v4[..];
    // let r5: &[usize] = &v5[..];

    // println!("{:_^22}", "unsigned");
    // println!("{:<12} | {:>3} | {:>3}", "u8",    size_of::<u8>(),    size_of_val(&x0));
    // println!("{:<12} | {:>3} | {:>3}", "u16",   size_of::<u16>(),   size_of_val(&x1));
    // println!("{:<12} | {:>3} | {:>3}", "u32",   size_of::<u32>(),   size_of_val(&x2));
    // println!("{:<12} | {:>3} | {:>3}", "u64",   size_of::<u64>(),   size_of_val(&x3));
    // println!("{:<12} | {:>3} | {:>3}", "u128",  size_of::<u128>(),  size_of_val(&x4));
    // println!("{:<12} | {:>3} | {:>3}", "usize", size_of::<usize>(), size_of_val(&x5));

    // println!("{:_^22}", "reference");
    // println!("{:<12} | {:>3} | {:>3}", "&u8",    size_of::<&u8>(),    size_of_val(&x0));
    // println!("{:<12} | {:>3} | {:>3}", "&u16",   size_of::<&u16>(),   size_of_val(&x1));
    // println!("{:<12} | {:>3} | {:>3}", "&u32",   size_of::<&u32>(),   size_of_val(&x2));
    // println!("{:<12} | {:>3} | {:>3}", "&u64",   size_of::<&u64>(),   size_of_val(&x3));
    // println!("{:<12} | {:>3} | {:>3}", "&u128",  size_of::<&u128>(),  size_of_val(&x4));
    // println!("{:<12} | {:>3} | {:>3}", "&usize", size_of::<&usize>(), size_of_val(&x5));

    println!("{:_^14}", "array");
    println!("{:<12} | {:>3} | {:>3}", "[u8;5]",    size_of::<[u8;5]>(),    size_of_val(&a0));
    println!("{:<12} | {:>3} | {:>3}", "[u16;5]",   size_of::<[u16;5]>(),   size_of_val(&a1));
    println!("{:<12} | {:>3} | {:>3}", "[u32;5]",   size_of::<[u32;5]>(),   size_of_val(&a2));
    println!("{:<12} | {:>3} | {:>3}", "[u64;5]",   size_of::<[u64;5]>(),   size_of_val(&a3));
    println!("{:<12} | {:>3} | {:>3}", "[u128;5]",  size_of::<[u128;5]>(),  size_of_val(&a4));
    println!("{:<12} | {:>3} | {:>3}", "[usize;5]", size_of::<[usize;5]>(), size_of_val(&a5));

    println!("{:_^14}", "pointer");
    println!("{:<12} | {:>3} | {:>3}", "&[u8; 5]",    size_of::<&[u8; 5]>(),    size_of_val(p0));
    println!("{:<12} | {:>3} | {:>3}", "&[u16; 5]",   size_of::<&[u16; 5]>(),   size_of_val(p1));
    println!("{:<12} | {:>3} | {:>3}", "&[u32; 5]",   size_of::<&[u32; 5]>(),   size_of_val(p2));
    println!("{:<12} | {:>3} | {:>3}", "&[u64; 5]",   size_of::<&[u64; 5]>(),   size_of_val(p3));
    println!("{:<12} | {:>3} | {:>3}", "&[u128; 5]",  size_of::<&[u128; 5]>(),  size_of_val(p4));
    println!("{:<12} | {:>3} | {:>3}", "&[usize; 5]", size_of::<&[usize; 5]>(), size_of_val(p5));

    println!("{:_^14}", "reference");
    println!("{:<12} | {:>3} | {:>3}", "&[u8]",    size_of::<&[u8]>(),    size_of_val(s0));
    println!("{:<12} | {:>3} | {:>3}", "&[u16]",   size_of::<&[u16]>(),   size_of_val(s1));
    println!("{:<12} | {:>3} | {:>3}", "&[u32]",   size_of::<&[u32]>(),   size_of_val(s2));
    println!("{:<12} | {:>3} | {:>3}", "&[u64]",   size_of::<&[u64]>(),   size_of_val(s3));
    println!("{:<12} | {:>3} | {:>3}", "&[u128]",  size_of::<&[u128]>(),  size_of_val(s4));
    println!("{:<12} | {:>3} | {:>3}", "&[usize]", size_of::<&[usize]>(), size_of_val(s5));

    // println!("{:_^14}", "vector");
    // println!("{:<12} | {:>3} | {:>3}", "Vec<u8>",    size_of::<Vec<u8>>(),    size_of_val(&v0));
    // println!("{:<12} | {:>3} | {:>3}", "Vec<u16>",   size_of::<Vec<u16>>(),   size_of_val(&v1));
    // println!("{:<12} | {:>3} | {:>3}", "Vec<u32>",   size_of::<Vec<u32>>(),   size_of_val(&v2));
    // println!("{:<12} | {:>3} | {:>3}", "Vec<u64>",   size_of::<Vec<u64>>(),   size_of_val(&v3));
    // println!("{:<12} | {:>3} | {:>3}", "Vec<u128>",  size_of::<Vec<u128>>(),  size_of_val(&v4));
    // println!("{:<12} | {:>3} | {:>3}", "Vec<usize>", size_of::<Vec<usize>>(), size_of_val(&v5));

    // println!("{:_^14}", "reference");
    // println!("{:<12} | {:>3} | {:>3}", "&[u8]",    size_of::<&[u8]>(),    size_of_val(&r0));
    // println!("{:<12} | {:>3} | {:>3}", "&[u16]",   size_of::<&[u16]>(),   size_of_val(&r1));
    // println!("{:<12} | {:>3} | {:>3}", "&[u32]",   size_of::<&[u32]>(),   size_of_val(&r2));
    // println!("{:<12} | {:>3} | {:>3}", "&[u64]",   size_of::<&[u64]>(),   size_of_val(&r3));
    // println!("{:<12} | {:>3} | {:>3}", "&[u128]",  size_of::<&[u128]>(),  size_of_val(&r4));
    // println!("{:<12} | {:>3} | {:>3}", "&[usize]", size_of::<&[usize]>(), size_of_val(&r5));

    // println!("u16: {}", size_of::<u16>());
    // println!("u32: {}", size_of::<u32>());
    // println!("u64: {}", size_of::<u64>());
    // println!("u128: {}", size_of::<u128>());
    // println!("usize: {}", size_of::<usize>());

    // println!("Vec<u8>: {}", size_of::<Vec<u8>>());
    // println!("Vec<u16>: {}", size_of::<Vec<u16>>());
    // println!("Vec<u32>: {}", size_of::<Vec<u32>>());
    // println!("Vec<u64>: {}", size_of::<Vec<u64>>());
    // println!("Vec<u128>: {}", size_of::<Vec<u128>>());
    // println!("Vec<usize>: {}", size_of::<Vec<usize>>());

}
