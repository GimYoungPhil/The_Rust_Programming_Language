#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// impl IpAddr {
//     fn from(kind: IpAddrKind, address: String) -> IpAddr {
//         IpAddr {
//             kind,
//             address,
//         }
//     }
// }

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    
    // route(four);
    // route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // let ip = IpAddr::from(IpAddrKind.V4, String::from("127.0.0.1"));
}

// fn route(ip_type: IpAddrKind) {
//     println!("{:?}", ip_type);
// }
