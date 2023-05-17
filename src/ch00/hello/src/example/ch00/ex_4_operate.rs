use std::mem::size_of;

pub fn run() -> () {

    let mut x0: i8 = 1;
    let x1: i8 = 2;

    println!("{:_^30}", "before");
    println!("{:<8} | {:>4} | {:p}", x0, size_of::<i8>(), &x0);
    println!("{:<8} | {:>4} | {:p}", x1, size_of::<i8>(), &x1);

    x0 = x0 + x1;
    println!("{:_^30}", "after");
    println!("{:<8} | {:>4} | {:p}", x0, size_of::<i8>(), &x0);
    println!("{:<8} | {:>4} | {:p}", x1, size_of::<i8>(), &x1);

    println!("{:_^30}", "end");
}
