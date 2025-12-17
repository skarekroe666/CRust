use rand::{Rng, rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");

    let secret_num = rng().random_range(1..=10);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
