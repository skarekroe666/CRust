// fn read(y: bool) {
//     if y {
//         println!("y is true");
//     }
// }

fn main() {
    // let x = 5;
    // let y = x;       //Copy

    let s1 = String::from("skarekroe");
    let s2 = s1;                                         //throws error
    // let s2 = s1.clone();
    // println!("{s1}");
    // println!("{s2}");

    let str = String::from("hello");
    takes_owner_ship(str);
    // println!("{str}");

    let (s3, len) = calculate_length(s2);
    println!("The length of {s3} is {len}");
}

fn takes_owner_ship(some_string: String) {
    println!("{some_string}");
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
