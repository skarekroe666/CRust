fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);

    let second = &v[1];
    println!("{}", second);

    let third = v.get(6);
    match third {
        Some(i) => println!("{i}"),
        None => println!("no number at tha index"),
    }

    let people = vec!["skarekroe", "sanjana", "anisha"];
    let third_wheel = &people.get(2);
    dbg!(third_wheel);

    for p in &people {
        println!("{p}");
    }

    let mut nums = vec![1, 2, 3];
    for n in &mut nums {
        *n += 20;
    }

    println!("nums in vector are: {:?}", nums);
}
