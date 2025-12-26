fn main() {
    let num = 6;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if num != 0 {
    //     println!("number was something other than zero");
    // }

    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("the value of num is {num}");

    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    println!("-------------------------------------");

    another_example();
    println!("-------------------------------------");
    another_while_loop();
    println!("-------------------------------------");
    another_for_example();
    println!("-------------------------------------");
    temp(43.5);
    println!("-------------------------------------");
}

fn another_example() {
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
}

fn another_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("the value is: {}", a[i]);
        i += 1;
    }
}

fn another_for_example() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("the value is: {i}");
    }

    for element in (1..4).rev() {
        println!("{element}")
    }
    println!("LIFTOFF!!!")
}

fn temp(t: f64) {
    let f = (t * 1.8) + 32.0;
    println!("The temp to Fahrenhiet is: {f}")
}
