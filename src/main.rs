#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ipv4 = IpAddress::V4;
    let ipv6 = IpAddress::V6;

    let v4 = ipv4(1, 2, 3, 4);
    let v6 = ipv6(String::from("::1"));

    println!("ipv4 is {:#?}", v4);
    println!("ipv6 is {:#?}", v6);
}
