fn main() {
    // print_the_measurements(5, 'm');
    println!("The sum of two numbers is: {}",sum());
    println!("The product is: {}", product(3,5));
}

/* fn print_the_measurements(value :i32, unit_label :char) {
     println!("The measurement is: {value}{unit_label}");
} */

fn sum() -> i32 {
    let x = 4;
    let y = 8;
    x + y
}

fn product(x :i32, y :i32) -> i32 {
    x * y
}
