fn main() {
    // let arr = [1, 2, 3, 4, 5];
    // let slice = &arr[1..3];
    // println!("The values in array: {:?}", slice);

    // let some_vec = vec![10, 20, 30, 40, 50];
    // let another_slice = &some_vec[2..4];
    // println!("The values in vectors: {another_slice:?}");

    // let s = String::from("hello sanjana");
    // let first_word = &s[0..5];
    // let second_word = &s[5..];
    // let length = s.len();
    // let slice = &s[6..length];
    // println!("{first_word}");
    // println!("{second_word}");
    // println!("{length}");
    // println!("{slice}");

    // let string = String::from("LY skarekroe");
    // let first_word = first_word(&string);
    // println!("{string}");
    // println!("{first_word}");

    // let s2 = "skarekroe sanjana";
    let s2 = String::from("skarekroe sanjana");
    let other_first_word = first_word(&s2);
    println!("{s2}");
    println!("{other_first_word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
