// use std::mem;

fn main() -> () {

    let v: Vec<i32> = vec![1, 2, 3];
    print_vec(&v);
    println!();

    // println!("{: <8}| {: >6} | {: >6} | {:#p}", "String", size_of::<String>(), size_of_val(&s), &s);

    // println!("{:_^37}", "literal string");
    // println!("{:_^8}|_{:_^6}_|_{:_^6}_|_{:_^18}", "Type", "Size", "Value", "Address");
    // println!("{: <8}| {: >6} | {: >6} | {:#p}",         "&str".red(),  size_of::<&str>(),  "-", &s);
    // println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&&str", size_of::<&&str>(), size_of_val(rs), rs, &rs);
    // println!("{: <8}| {: >6} | {: >6} | {:#p} | {:#p}", "&str",  size_of::<&str>(),  size_of_val(rs), ss, &ss);
    // println!();

}

fn print_vec<T>(v: &Vec<T>) {
    println!("|{:^15p}", v);
    println!("|{:_^15}|{:_^15}|{:_^15}|", "pointer", "capacity", "length");
    println!("|{:^15p}|{:^15X}|{:^15X}|", v.as_ptr(), v.len(), v.capacity());
    // println!("{:p}", v.as_ptr());
    // println!("{:X}", v.len());
    // println!("{:X}", v.capacity());
}
