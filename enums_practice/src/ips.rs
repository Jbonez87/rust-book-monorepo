#[derive(Debug)]
pub enum IpAddrKind {
    V6,
    V4,
}

// example struct
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum IpAddr {
    // String example
    // V4(String),
    // V6(String),
    V4(u8, u8, u8, u8),
    V6(String),
}


pub fn test_ips() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    //struct examples
    // let home: IpAddr = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback: IpAddr = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}