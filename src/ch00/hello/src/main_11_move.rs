use std::mem::{size_of};

fn main() -> () {

    let v: Vec<u32> = vec![1, 2, 3];
    // let rv: &Vec<u32> = &v;
    // let sv: &[u32] = &v[..];
    println!("{:_^13}|{:_^8}|{:_^12}", "Type", "size", "address");
    println!("{:<12} | {:>6} | {:p}", "Vec<u32>",    size_of::<Vec<u32>>(),    &v);

    let c: Vec<u32> = v;
    println!("{:_^13}|{:_^8}|{:_^12}", "Type", "size", "address");
    println!("{:<12} | {:>6} | {:p}", "Vec<u32>",    size_of::<Vec<u32>>(),    &c);

    println!("{:?}", c);
}

