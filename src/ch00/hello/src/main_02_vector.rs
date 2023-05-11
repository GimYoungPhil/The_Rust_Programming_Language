use std::mem::{size_of, size_of_val};

fn main() -> () {

    let v0: Vec<i8>    = vec![1, 2, 3, 4, 5];
    let v1: Vec<i16>   = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32>   = vec![1, 2, 3, 4, 5];
    let v3: Vec<i64>   = vec![1, 2, 3, 4, 5];
    let v4: Vec<i128>  = vec![1, 2, 3, 4, 5];
    let v5: Vec<isize> = vec![1, 2, 3, 4, 5];

    let r0: &Vec<i8>    = &v0;
    let r1: &Vec<i16>   = &v1;
    let r2: &Vec<i32>   = &v2;
    let r3: &Vec<i64>   = &v3;
    let r4: &Vec<i128>  = &v4;
    let r5: &Vec<isize> = &v5;

    let s0: &[i8]    = &v0[..];
    let s1: &[i16]   = &v1[..];
    let s2: &[i32]   = &v2[..];
    let s3: &[i64]   = &v3[..];
    let s4: &[i128]  = &v4[..];
    let s5: &[isize] = &v5[..];

    println!("{:_^30}", "Vector");
    println!("{:_^13}|{:_^8}|{:_^12}", "Type", "size", "address");
    println!("{:<12} | {:>6} | {:p}", "Vec<i8>",    size_of::<Vec<i8>>(),    &v0);
    println!("{:<12} | {:>6} | {:p}", "Vec<i16>",   size_of::<Vec<i16>>(),   &v1);
    println!("{:<12} | {:>6} | {:p}", "Vec<i32>",   size_of::<Vec<i32>>(),   &v2);
    println!("{:<12} | {:>6} | {:p}", "Vec<i64>",   size_of::<Vec<i64>>(),   &v3);
    println!("{:<12} | {:>6} | {:p}", "Vec<i128>",  size_of::<Vec<i128>>(),  &v4);
    println!("{:<12} | {:>6} | {:p}", "Vec<isize>", size_of::<Vec<isize>>(), &v5);

    println!("{:_^30}", "reference Vector");
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&Vec<i8>",    size_of::<&Vec<i8>>(),    size_of_val(r0), r0, &r0);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&Vec<i16>",   size_of::<&Vec<i16>>(),   size_of_val(r1), r1, &r1);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&Vec<i32>",   size_of::<&Vec<i32>>(),   size_of_val(r2), r2, &r2);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&Vec<i64>",   size_of::<&Vec<i64>>(),   size_of_val(r3), r3, &r3);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&Vec<i128>",  size_of::<&Vec<i128>>(),  size_of_val(r4), r4, &r4);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&Vec<isize>", size_of::<&Vec<isize>>(), size_of_val(r5), r5, &r5);

    println!("{:_^30}", "reference slice Vector");
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i8]",    size_of::<&[i8]>(),    size_of_val(s0), s0, &s0);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i16]",   size_of::<&[i16]>(),   size_of_val(s1), s1, &s1);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i32]",   size_of::<&[i32]>(),   size_of_val(s2), s2, &s2);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i64]",   size_of::<&[i64]>(),   size_of_val(s3), s3, &s3);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[i128]",  size_of::<&[i128]>(),  size_of_val(s4), s4, &s4);
    println!("{:<12} | {:>4} | {:>4} | {:p} | {:p}", "&[isize]", size_of::<&[isize]>(), size_of_val(s5), s5, &s5);

    println!("{:_^30}", "end");
}
