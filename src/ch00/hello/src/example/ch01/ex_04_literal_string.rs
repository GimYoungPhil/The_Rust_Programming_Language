use std::mem::{size_of, size_of_val};

pub fn run() -> () {

    let s: &str = "Rust";
    let rs: &&str = &s;
    let ss: &str = &s[..];

    println!("{:_^37}", "literal string");
    println!("{:_^8}|_{:_^6}_|_{:_^6}_|_{:_^18}", "Type", "Size", "Value", "Address");
    println!("{: <8}| {: >6} | {: >6} | {:#p}",         "&str",  size_of::<&str>(),  "-", &s);
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&&str", size_of::<&&str>(), size_of_val(rs), rs, &rs);
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&str",  size_of::<&str>(),  size_of_val(rs), ss, &ss);
    println!();

}
