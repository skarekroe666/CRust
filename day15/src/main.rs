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
    let another_home = IpAddrKind::V6(String::from("::01"));
    println!("{:?}", another_home);

    let q = QuitMessage;
    let m = MoveMessage{x: 54, y: 32};
    let w = WriteMessage(String::from("Hello Skarekroe"));
    let c = ChangeColor(0, 255, 0);

    println!("{:?}", q);
    println!("{:?}", m);
    println!("{:?}", w);
    println!("{:?}", c);
}

// fn route(ip_type: IpAddrKind) {
//     println!("{:?}", ip_type)
// }

#[derive(Debug)]
struct QuitMessage;

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String);

#[derive(Debug)]
struct ChangeColor(i32, i32, i32);
