use std::io::stdin;

const SPEED_OF_SOUND: u32 = 343;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    {
        let x = 6 * x;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("The speed of sound is: {SPEED_OF_SOUND}mps");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
    println!("---------------------------------------------");

    data_types();
    floating();
    characters();
    compound();
    arrays();
}

fn data_types() {
    let num: u32 = "42".parse().expect("Not a number!");
    println!("{num}");
    println!("---------------------------------------------");
}

fn floating() {
    let a = 2.4;
    let b: f32 = 2.5;
    println!("The value of a is: {a}, b: {b}");

    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncated = -5 / 2; // Results in -1
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");
    println!("---------------------------------------------");
}

fn characters() {
    let c = 'z';
    println!("{c}");
    let z: char = 'Z';
    println!("{z}");
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
    println!("---------------------------------------------");
}
fn compound() {
    let tup = (500, 2.4, true);

    // let (x,y,z) = tup;
    // println!("the value of x is: {x}");
    // println!("the value of z is: {z}");

    let num = tup.0;
    let point = tup.1;
    let ans = tup.2;

    println!("num: {num}, point: {point}, val: {ans}");

    let mut x = (1, 2);
    x.0 = 0;
    x.1 += 2;
    print!("{0} {1}\n", x.0, x.1);
    println!("---------------------------------------------");
}

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let z = [3; 5];

    println!("{}, {}, {}", a[0], a[3], a[1]);
    println!("{}, {}, {}", b[0], b[3], b[1]);
    println!("{:?}", z);
    println!("---------------------------------------------");

    let arr = [1,2,3,4,5];
    let mut index = String::new();

    stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number!");

    let element = arr[index];
    println!("The value of the element at the index {index} is: {element}");

    println!("---------------------------------------------");
}
