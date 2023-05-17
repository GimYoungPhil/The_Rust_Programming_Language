use std::mem::{size_of};

fn main() -> () {

    let mut v: Vec<u32> = vec![1, 2, 3];
    // let rv: &Vec<u32> = &v;
    // let sv: &[u32] = &v[..];
    println!("{:_^13}|{:_^8}|{:_^15}|{:_^15}|{:_^15}|{:_^15}|", "Type", "size", "address", "pointer", "capacity", "length");
    println!("{:<12} | {:>6} |{:^15p}|{:^15p}|{:^15X}|{:^15X}|", "Vec<u32>",    size_of::<Vec<u32>>(),    &v, v.as_ptr(), v.len(), v.capacity());

    let c: Vec<u32> = v;
    println!("{:<12} | {:>6} |{:^15p}|{:^15p}|{:^15X}|{:^15X}|", "Vec<u32>",    size_of::<Vec<u32>>(),    &c, c.as_ptr(), c.len(), c.capacity());

    v = vec![5, 6, 7, 8];
    println!("{:<12} | {:>6} |{:^15p}|{:^15p}|{:^15X}|{:^15X}|", "Vec<u32>",    size_of::<Vec<u32>>(),    &v, v.as_ptr(), v.len(), v.capacity());

    println!("{:?}", c);
    println!("{:?}", v);
}
