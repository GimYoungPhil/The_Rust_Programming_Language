#[derive(Debug)]
struct Ipv4Addr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(Debug)]
struct Ipv6Addr {
    a: u16,
    b: u16,
    c: u16,
    d: u16,
    e: u16,
    f: u16,
    g: u16,
    h: u16,
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {

    let v4 = Ipv4Addr {
        a: 127,
        b: 0,
        c: 0,
        d: 1,
    };
    let v6 = Ipv6Addr {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0xffff,
        g: 0xc00a,
        h: 0x2ff,
    };

    let home = IpAddr::V4(v4);
    let loopback = IpAddr::V6(v6);

    println!("{:#?}", home);
    println!("{:#?}", loopback);
}
