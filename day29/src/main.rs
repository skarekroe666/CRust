use std::fmt::{self, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news!! {}", item);
}

pub fn notify_longform<T: Summary + Display>(item: &T) {
    println!("Breaking news!! {}", item);
}

struct BlogPost {
    title: String,
    author: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl fmt::Display for BlogPost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlogPost: {} byt {}", self.title, self.author)
    }
}

fn main() {
    let post1 = BlogPost {
        title: String::from("Rust Traits"),
        author: String::from("Alice"),
    };

    let post2 = BlogPost {
        title: String::from("Learning Rust"),
        author: String::from("Bob"),
    };

    notify(&post1);
    notify_longform(&post2);
}
