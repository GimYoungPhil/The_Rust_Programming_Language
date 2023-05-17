#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct Ip {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    let home = Ip {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let work = Ip {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:p}", &home);
    println!("{:p}", &work);

    println!("{:?}", home);
    println!("{:?}", work);
}
