use std::mem::{size_of, size_of_val};

fn main() -> () {

    let s: String = String::from("Rust");

    println!("{: <8}| {: >6} | {: >6} | {:#p}", "String", size_of::<String>(), size_of_val(&s), &s);

    let len: usize = calculate_length(&s);

    println!("{}", len);
    // let rs: &String = &s;
    // let ss: &str = &s[..];

    // println!("{:_^37}", "literal string");
    // println!("{:_^8}|_{:_^6}_|_{:_^6}_|_{:_^18}", "Type", "Size", "Value", "Address");
    // println!("{: <8}| {: >6} | {: >6} | {:#p}",         "&str".red(),  size_of::<&str>(),  "-", &s);
    // println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&&str", size_of::<&&str>(), size_of_val(rs), rs, &rs);
    // println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&str",  size_of::<&str>(),  size_of_val(rs), ss, &ss);
    // println!();

}

fn calculate_length(rs: &String) -> usize {
    println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&String", size_of::<&String>(), size_of_val(rs), rs, &rs);
    rs.len()
}
