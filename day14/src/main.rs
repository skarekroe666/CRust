#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 5,
    };
    println!("The area of the rectangle is {}", area(&rect1));
    println!("{rect1:?}");
    println!("{rect1:#?}");

    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

