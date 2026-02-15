use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, skarekroe!");

    let mut scores = HashMap::new();
    scores.insert(String::from("skarekroe"), 69);
    scores.insert(String::from("sanjana"), 84);
    // dbg!(scores);

    let new_scores = scores.get("skarekroe").copied().unwrap_or(0);
    dbg!(new_scores);

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    println!("---------------------------------------------------");

    let mut items = HashMap::new();
    items.insert("cup", 10);
    // dbg!(items);

    items.entry("cup").or_insert(20);
    items.entry("fork").or_insert(20);
    println!("{:?}", items);

    println!("---------------------------------------------------");

    hash_set();
}

fn hash_set() {
    // let mut nums = HashSet::new();
    // nums.insert("10");
    // nums.insert("20");
    // nums.insert("10");
    let nums = HashSet::from([10, 20, 10, 30, 40]);
    println!("{:?}", nums);

    // dbg!(nums.contains(&20));

    println!("---------------------------------------------------");

    dbg!(nums.is_empty());

    for n in &nums {
        println!("{:?}", n);
    }

    println!("---------------------------------------------------");

    let hs1 = HashSet::from([1, 2, 3, 4, 5]);
    let hs2 = HashSet::from([4, 5, 6, 6, 7, 8]);

    // let result: HashSet<&i32> = hs1.union(&hs2).collect();
    // let result: HashSet<&i32> = hs1.intersection(&hs2).collect();
    let result: HashSet<&i32> = hs1.difference(&hs2).collect();
    println!("{:?}", result);
}
