enum ipAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ipv4 = ipAddress::V4;
    let ipv6 = ipAddress::V6;
}
