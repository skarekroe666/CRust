const SPEED_OF_SOUND: u32 = 330;

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

    data_types();
}

fn data_types() {
    let num :u32 = "42".parse().expect("Not a number!");
    println!("{num}");
}
