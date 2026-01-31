fn main() {
    let mut x = 5;
    increment(&mut x);
    println!("{x}");
}

fn increment(x: &mut i32) {
    *x += 1;
}
