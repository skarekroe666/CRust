#[allow(dead_code)]
enum Coin {
    Penny(UsState),
    Nickel,
    Dime,
    Quarter(Rarity),
}
#[derive(Debug)]
enum UsState {
    Arizona,
    California,
    Colorado,
    Washington,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny(state) => {
            println!("This penny is from {:?}", state);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(rarity) => {
            println!("You got a {:?} quarter!", rarity);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny(UsState::California);
    println!("The value of th penny is {}", value_in_cents(penny));

    let quarter = Coin::Quarter(Rarity::Epic);
    println!("The value of the quarter is {}", value_in_cents(quarter));

    println!("--------------------------------------------------------------------");

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);

    let no_value: Option<i32> = None;
    let result = plus_one(no_value);
    println!("{:?}", result);

    println!("--------------------------------------------------------------------");

    let dice_roll = 3;
    match dice_roll {
        3 => add_hp(),
        6 => remove_hp(),
        _ => (),
    }

    // println!("You got: {}", dice_roll);
}

fn add_hp() {
    println!("you got extra hp");
}

fn remove_hp() {
    println!("you lost some hp");
}
