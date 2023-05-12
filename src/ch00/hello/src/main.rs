fn main() -> () {

    let tup: (u32, f32, u32) = (1, 0.5, 3);
    let rt: &(u32, f32, u32) = &tup;

    println!("{:?}", tup);
    println!("{:?}", rt);
    println!("end...");

}
