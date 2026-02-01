fn main() {
    let r = Color::Green;
    print_color(&r);

    let result = r.green_part();
    // let is_green_result = r.is_green();
    let is_green_result = Color::is_green(&r);
    println!("{:?}", result);
    println!("{:?}", is_green_result);

    println!("------------------------------------------------");

    let foo = Item::Foo(String::from("skarekroe"));
    if let Item::Foo(s) = foo {
        println!("{}", s);
    }

    let baz = Item::Baz(Custom {
        name: String::from("skarekroe"),
        age: 27,
    });
    if let Item::Baz(c) = baz {
        println!("{:?}", c);
    }
}

enum Color {
    Yellow,
    Blue,
    Red,
    Green,
}

fn print_color(color: &Color) {
    match color {
        Color::Yellow => println!("Yellow"),
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
    }
}

impl Color {
    fn green_part(&self) -> bool {
        match self {
            Color::Blue => true,
            Color::Yellow => true,
            _ => false,
        }
    }

    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
}

#[derive(Debug)]
struct Custom {
    name: String,
    age: usize,
}
#[derive(Debug)]
enum Item {
    Foo(String),
    Bar(usize),
    Baz(Custom),
}
