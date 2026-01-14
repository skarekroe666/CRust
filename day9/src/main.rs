use std::vec;

fn main() {
    // Basic string borrowing example
    let s1 = String::from("Skarekroe");
    borrow_value(&s1);
    println!("------------------------------------------------------");

    // Mutable reference to an integer
    let mut b = 10;
    let c = &mut b;
    *c += 10;
    println!("{c}");
    println!("------------------------------------------------------");

    // Mutable string update
    let mut greet = String::from("Hello");
    println!("Before update: {greet}");
    update_word(&mut greet);
    println!("After update: {greet}");
    println!("------------------------------------------------------");

    // Another string reference example
    let mut a1 = String::from("Some string");
    println!("{a1}");
    let a2 = &mut a1;
    println!("{a2}");
    println!("------------------------------------------------------");

    // Function returning multiple values
    let m1 = String::from("Sanjana");
    let m2 = String::from("loml");
    let (m1_again, m2_again) = greet_someone(m1, m2);
    let _s = format!("{m1_again} {m2_again}");
    println!("------------------------------------------------------");

    // Dereferencing pointers and Box
    deref_pointer();
    println!("------------------------------------------------------");

    // More examples with Box and references
    some_more_example();
    println!("------------------------------------------------------");

    // Aliasing and borrowing rules
    aliasing();
}

fn borrow_value(some_str: &String) {
    println!("Borrowed value: {some_str}");
}

fn update_word(word: &mut String) {
    word.push_str(" Sanjana");
}

fn greet_someone(g1: String, g2: String) -> (String, String) {
    println!("Greeting: {g1} {g2}");
    (g1, g2)
}

fn deref_pointer() {
    let mut x = Box::new(11);
    let a = *x; // *x reads the heap value, so a = 11
    *x += 1; // *x on the left-side modifies the heap value, so x points to the value 12

    let r1 = &x; // r1 points to x on the stack
    let b = **r1; // two dereferences get us to the heap value

    let r2 = &*x; // r2 points to the heap value directly
    let c = *r2; // so only one dereference is needed to read it

    println!("x: {x}, a: {a}");
    println!("r1: {r1}, b: {b}");
    println!("r2: {r2}, c: {c}");
}

fn some_more_example() {
    let x = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Waddup");
    let s_len1 = str::len(&s);
    let s_len2 = str::len(&s);
    assert_eq!(s_len1, s_len2);
}

fn aliasing() {
    let mut v = vec![1, 2, 3];
    // The following would cause a compile error because we can't have
    // a mutable borrow while there's an immutable borrow active
    // let num = &v[2];
    // v.push(4);
    // println!("Third element is {}", *num);

    // Instead, we can do this:
    v.push(4);
    println!("Vector after push: {:?}", v);
}
