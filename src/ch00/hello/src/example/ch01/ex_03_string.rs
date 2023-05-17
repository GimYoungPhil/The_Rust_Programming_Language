use std::mem::{size_of, size_of_val};

pub fn run() -> () {

    let s: String = String::from("Rust");
    let rs: &String = &s;
    let ss: &str = &s[..];

    println!("{:_^37}", "string");
    println!("{:_^8}|_{:_^6}_|_{:_^6}_|_{:_^18}", "Type", "Size", "Value", "Address");
    println!("{: <8}| {: >6} | {: >6} | {:#p}", "String",  size_of::<String>(), "-", &s);
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&String", size_of::<&String>(), size_of_val(rs), rs, &rs);
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&str",    size_of::<&str>(),    size_of_val(rs), ss, &ss);
    println!();

}
