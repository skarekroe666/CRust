#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // println!("{:?}", four);
    // println!("{:?}", six);

    // route(four);
    // route(six);

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("{:?}", home);
}

// fn route(ip_type: IpAddrKind) {
//     println!("{:?}", ip_type)
// }

