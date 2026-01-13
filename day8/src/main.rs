fn main() {
    // let m1 = String::from("Hello");
    // let m2 = String::from("Skarekroe");                                 //L1
    // let (m1_again, m2_again) = greet(m1, m2);
    // let _s = format!("{} {}", m1_again, m2_again);                  //L2

    let m1 = String::from("Hello");
    let m2 = String::from("Skarekroe");
    greet(&m1, &m2);
    println!("----------------------------------------");

    stack_fn();
    heap_fn();
    update_string();
    println!("----------------------------------------");
    let str = String::from("Sanjana");
    takes_ownership(str);
}

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{g1}, {g2}");
//     (g1, g2)
// }

fn greet(g1: &String, g2: &String) {
    println!("{g1} {g2}");
}

fn stack_fn() {
    let a = 20;
    let b = 30;
    let c = a + b;
    println!("Stack funtion: The addition of {a} and {b} is {c}");
}

fn heap_fn() {
    let str1 = String::from("hello");
    let str2 = String::from("skarekroe");
    println!("Heap function: Combined string is {str1} {str2}");
}

fn update_string() {
    let mut s = String::from("hello");
    println!("Before update: {s}");
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    println!("----------------------------------------");

    s.push_str(" skarekroe");
    println!("After update: {s}");
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}

fn takes_ownership(fn_str: String) {
    println!("{fn_str}");
}
