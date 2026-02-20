use std::fmt::Display;

fn main() {
    //&i32        // a reference
    //&'a i32     // a reference with an explicit lifetime
    //&'a mut i32 // a mutable reference with an explicit lifetime

    let str1 = String::from("hey, sanjana");
    let str2 = String::from("wanna come over");

    let result = longest(str1.as_str(), str2.as_str());
    dbg!(result);

    println!("----------------------------------------------------");

    let novel = String::from("The Secret. By Rhonda Bryne");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImpExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    dbg!(i.level());

    dbg!(i.annouce_and_return("Everyone"));

    println!("----------------------------------------------------");

    let _rule1 = first_word("rule 1");
    let _rule2 = second_word("rule 2");

    println!("----------------------------------------------------");

    let _static_str: &'static str = "i'm static stirng";

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let annoucement =
        longest_with_annoucement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {annoucement}");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

#[derive(Debug)]
struct ImpExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImpExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
impl<'a> ImpExcerpt<'a> {
    fn annouce_and_return(&self, announce: &str) -> &str {
        println!("Attention please: {announce}");
        self.part
    }
}

fn first_word<'a>(s: &'a str) -> &'a str {
    s
}

fn second_word<'a>(s: &'a str) -> &'a str {
    s
}

// fn third_word<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

fn longest_with_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoucment! {ann}");
    if x.len() > y.len() { x } else { y }
}
