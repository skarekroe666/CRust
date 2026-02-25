use std::ops::Deref;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let box1 = Box::new(5);
    println!("b = {box1}");

    // let list = List::Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    dbg!(assert_eq!(5, x));
    dbg!(assert_eq!(5, *y));

    println!("------------------------------------------------------------------");

    let b1 = 5;

    dbg!(assert_eq!(5, b1));
}

#[derive(Debug)]
struct Mybox<T>(T);

impl<T> Deref for Mybox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
