use std::mem::{size_of, size_of_val};
use colored::Colorize;

pub fn run() -> () {

    let s: &str = "러스트";
    let rs: &&str = &s;
    let ss: &str = &s[0..3];

    println!("{:_^37}", "literal string");
    println!("{:_^8}|_{:_^6}_|_{:_^6}_|_{:_^18}", "Type", "Size", "Value", "Address");
    println!("{: <8}| {: >6} | {: >6} | {:#p}",         "&str".red(),  size_of::<&str>(), s.len(), &s);
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&&str", size_of::<&&str>(), size_of_val(rs), rs, &rs);
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&str",  size_of::<&str>(),  size_of_val(rs), ss, &ss);
    println!();

}
