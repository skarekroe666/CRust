#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny(Metal),
    Nickel,
    Dime,
    Quarter(Metal),
}

#[derive(Debug)]
enum Metal {
    Copper,
    Silver,
    Steel,
    Iron,
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let max_config = Some(3u8);
    if let Some(max) = max_config {
        println!("The maximum is configured to be {max}");
    }

    println!("--------------------------------------------------------------");

    println!("----------------MATCH STATEMENT----------------");
    let dice_roll = 1;
    match dice_roll {
        1 => println!("You got 1"),
        2 => (),
        3 => (),
        4 => (),
        5 => (),
        6 => println!("You got 6"),
        _ => (),
    }

    println!("----------------IF LET STATEMENT----------------");

    let another_dice_roll: Option<i32> = None;
    if let Some(num) = another_dice_roll {
        println!("You got {}", num)
    } else {
        println!("ERROR: None");
    }

    println!("--------------------------------------------------------------");

    let coin1 = Coin::Quarter(Metal::Silver);
    if let Coin::Quarter(metal) = coin1 {
        println!("This quarter is made of {:?}", metal);
    } else {
        println!("This is not a quarter, this is a {:?}", coin1);
    }

    let coin2 = Some(Coin::Penny(Metal::Copper));
    if let Some(Coin::Penny(metal)) = coin2 {
        println!("This penny is made of {:?}", metal);
    } else {
        println!("No penny found");
    }

    let coin3: Option<Coin> = None;
    if let Some(Coin::Nickel) = coin3 {
        ()
    }

    println!("--------------------------------------------------------------");

    let nickname: Option<String> = Some("skarekroe".to_string());
    if let Some(name) = nickname {
        println!("Welcome back, {name}!");
    }

}
