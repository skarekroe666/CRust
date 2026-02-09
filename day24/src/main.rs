use std::collections::HashMap;

fn main() {
    let s = "sanjana, skarekroe's hoe".to_string();
    println!("{s}");

    let mut s1 = String::from("sanjana");
    s1.push_str(" the sexiest cougar");
    println!("{s1}");

    let s2 = String::from("waddup");
    let slice = &s2[2..4];
    println!("Sliced string: {slice}");

    for c in s2.chars() {
        println!("{c}");
    }

    let s3 = "welcome to london";
    let new_s3 = s3.replace("london", "miami");
    println!("New String: {new_s3}");

    println!("-----------------------------------------------------");

    let mut score = HashMap::new();
    score.insert(String::from("blue"), 21);
    // map1.insert(String::from("yellow"), 45);

    let team_score = score.get("blue").copied().unwrap_or(0);
    println!("Blue team score: {:?}", team_score);

    score.insert(String::from("blue"), 69);
    score.entry(String::from("yellow")).or_insert(76);
    score.entry(String::from("blue")).or_insert(42);
    println!("{:?}", score);

    // for (key, value) in &score {
    //     println!("{key}: {value}");
    // }

    println!("-----------------------------------------------------");

    let field_name = String::from("partner");
    let field_value = String::from("sanjana");

    let mut map2 = HashMap::new();
    map2.insert(field_name, field_value);
    // map2.insert(&field_name, &field_value);

    println!("{:?}", map2);

    println!("-----------------------------------------------------");

    let text = "hello world beautiful world";
    let mut map3 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map3);
}
