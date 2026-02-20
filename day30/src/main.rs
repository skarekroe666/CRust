// fn func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     unimplemented!();
// }
//
// fn func2<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!();
// }

use std::fmt::Display;

fn main() {
    let item = return_summarizable();
    println!("{:?}", item.summarize());

    println!("----------------------------------------------------");

    let something = Pair::init(32, 64);
    dbg!(&something);
    something.cmd_display();

    let s = 43.to_string();
    dbg!(s);
}

trait Summary {
    fn summarize(&self) -> String;
}

struct SocialPost {
    username: String,
    content: String,
}

fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("sanjana"),
        content: String::from("she's the best"),
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn init(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest number is {}", self.x);
        } else {
            println!("The largest number is {}", self.y);
        }
    }
}
