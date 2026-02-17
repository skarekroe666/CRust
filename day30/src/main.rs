// fn func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     unimplemented!();
// }

// fn func2<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!();
// }

fn main() {
    let item = return_summarizable();
    println!("{:?}", item.summarize());
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
