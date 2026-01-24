#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let q = Message::Quit;
    let w = Message::Write(String::from("hello"));
    let m = Message::Move { x: 32, y: 43 };
    let c = Message::ChangeColor(122, 0, 0);
    q.call();
    w.call();
    m.call();
    c.call();

    println!("---------------------------------------------------------");

    let some_num = Some(5);
    let some_string = Some("a string");
    let absent_num: Option<i32> = None;

    println!("{:?}", some_num);
    println!("{:?}", some_string);
    println!("{:?}", absent_num);

    println!("---------------------------------------------------------");

    let x: i8 = 5;
    let y: Option<i8> = Some(10);

    //unwrap() is a method that returns the value inside the Option<T> if it is Some<T>
    let sum = x + y.unwrap();
    println!("{sum}");
}

