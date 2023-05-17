use std::mem::{size_of, size_of_val};

pub fn run() -> () {

    let a0: [i8; 5]    = [1, 2, 3, 4, 5];
    let a1: [i16; 5]   = [1, 2, 3, 4, 5];
    let a2: [i32; 5]   = [1, 2, 3, 4, 5];
    let a3: [i64; 5]   = [1, 2, 3, 4, 5];
    let a4: [i128; 5]  = [1, 2, 3, 4, 5];
    let a5: [isize; 5] = [1, 2, 3, 4, 5];

    let r0: &[i8; 5]    = &a0;
    let r1: &[i16; 5]   = &a1;
    let r2: &[i32; 5]   = &a2;
    let r3: &[i64; 5]   = &a3;
    let r4: &[i128; 5]  = &a4;
    let r5: &[isize; 5] = &a5;

    let s0: &[i8]    = &a0[..];
    let s1: &[i16]   = &a1[..];
    let s2: &[i32]   = &a2[..];
    let s3: &[i64]   = &a3[..];
    let s4: &[i128]  = &a4[..];
    let s5: &[isize] = &a5[..];

    println!("{:_^30}", "array");
    println!("{:<12} | {:>4} | {:>4} | {:p}", "i8",    size_of::<[i8; 5]>(),    size_of_val(&a0), &a0);
    println!("{:<12} | {:>4} | {:>4} | {:p}", "i16",   size_of::<[i16; 5]>(),   size_of_val(&a1), &a1);
    println!("{:<12} | {:>4} | {:>4} | {:p}", "i32",   size_of::<[i32; 5]>(),   size_of_val(&a2), &a2);
    println!("{:<12} | {:>4} | {:>4} | {:p}", "i64",   size_of::<[i64; 5]>(),   size_of_val(&a3), &a3);
    println!("{:<12} | {:>4} | {:>4} | {:p}", "i128",  size_of::<[i128; 5]>(),  size_of_val(&a4), &a4);
    println!("{:<12} | {:>4} | {:>4} | {:p}", "isize", size_of::<[isize; 5]>(), size_of_val(&a5), &a5);

    println!("{:_^30}", "reference array");
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i8; 5]",    size_of::<&[i8; 5]>(),    size_of_val(r0), r0, &r0);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i16; 5]",   size_of::<&[i16; 5]>(),   size_of_val(r1), r1, &r1);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i32; 5]",   size_of::<&[i32; 5]>(),   size_of_val(r2), r2, &r2);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i64; 5]",   size_of::<&[i64; 5]>(),   size_of_val(r3), r3, &r3);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i128; 5]",  size_of::<&[i128; 5]>(),  size_of_val(r4), r4, &r4);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[isize; 5]", size_of::<&[isize; 5]>(), size_of_val(r5), r5, &r5);

    println!("{:_^30}", "reference slice array");
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i8]",    size_of::<&[i8]>(),    size_of_val(s0), s0, &s0);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i16]",   size_of::<&[i16]>(),   size_of_val(s1), s1, &s1);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i32]",   size_of::<&[i32]>(),   size_of_val(s2), s2, &s2);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i64]",   size_of::<&[i64]>(),   size_of_val(s3), s3, &s3);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i128]",  size_of::<&[i128]>(),  size_of_val(s4), s4, &s4);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[isize]", size_of::<&[isize]>(), size_of_val(s5), s5, &s5);

    println!("{:_^30}", "end");
}
