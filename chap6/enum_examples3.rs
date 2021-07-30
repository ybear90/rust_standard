// 각 variant는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있음
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
