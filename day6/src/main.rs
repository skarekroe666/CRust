fn main() {
    // let num = 6;

    // if num < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    // println!("-----------------------------------");

    // let condition = false;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of the number is {number}");
    // println!("-----------------------------------");

    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;

    // loop {
    //     counter += 1;
    //     println!("{counter}");

    //     if counter == 10 {
    //         break;
    //     }
    // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("-----------------------------------");

    while_loop();
    println!("-----------------------------------");

    while_array();
    println!("-----------------------------------");

    for_loop();
    println!("-----------------------------------");

    another_for_loop();
    println!("-----------------------------------");
}

fn while_loop() {
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF!!")
}

fn while_array() {
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("the value is: {}", a[i]);
        i += 1;
    }
}

fn for_loop() {
    let arr = ['a', 'b', 'c', 'd', 'e'];

    for i in arr {
        println!("the value is {i}");
    }
}

fn another_for_loop() {
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("LIFTOFF");
}


