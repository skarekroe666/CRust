pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// impl Summary for NewArticle {
//     fn summarize(&self) -> String {
//         format!("{} by {}({})", self.headline, self.author, self.location)
//     }
// }
//
// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

impl Summary for NewsArticle {}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is Great"),
        location: String::from("London"),
        author: String::from("John Doe"),
        content: String::from("Rust is a systems programming language..."),
    };

    let post = SocialPost {
        username: String::from("rust_lover"),
        content: String::from("Just learned about traits in Rust!"),
        reply: false,
        repost: false,
    };

    println!("{}", article.summarize());
    println!("{}", post.summarize());
}
