fn main() {
    let mut s1 = String::from("hello");
    change( &mut s1);
    println!("{s1}");

    let sentence = "waddup skarekroe";
    let first_word = &sentence[0..6];
    let second_word = &sentence[7..];

    println!("{first_word}");
    println!("{second_word}");

    let arr = vec![1, 2, 3, 4, 5];
    let slice = &arr[0..3];
    println!("{:?}", slice);

    let some_str = String::from("skarekroe");
    let length = calculate_len(&some_str);
    println!("the length of {some_str} is: {length}");
}

fn change(str: &mut String) {
    str.push_str(" sanjana");
}

fn calculate_len(s: &String) -> usize {
    let len = s.len();
    len
}

