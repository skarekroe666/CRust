struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 6,
        width: 10,
    };

    // println!("The area of the rectangle is {}", rect1.area());
    if rect1.width() {
        println!("The area of the rectangle is {}", Rectangle::area(&rect1));
    } else {
        println!("ERROR: The value is 0");
    }

    let rect2 = Rectangle {
        height: 4,
        width: 6,
    };
    println!("can hold: {}", rect1.can_hold(&rect2));

    println!("----------------------------------------------------");

    let sq = Rectangle::square(9);
    println!("The area of Square is {}", sq.area());

    println!("----------------------------------------------------");

    let user1 = User {
        name: String::from("skarekroe"),
        partner: String::from("sanjana"),
    };

    // user1.info();
    User::info(&user1);
}

struct User {
    name: String,
    partner: String,
}

impl User {
    fn info(&self) {
        println!("{} loves {}", self.partner, self.name);
    }
}
