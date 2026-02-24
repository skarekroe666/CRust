pub mod art;
use crate::art::{kinds::PrimaryColor, utils::mix};

fn main() {
    println!("Hello, world!");
    let result = add_one(43);
    dbg!(result);

    println!("-----------------------------------------------------");

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
