use std::collections::HashMap;

fn main() {
    println!("Hello, skarekroe!");

    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("{:?}\n", map);

    // let mut count = 11;

    // while count > 1 {
    //     count -= 1;

    //     if count % 2 == 0 {
    //         continue;
    //     }

    //     println!("{count}");
    // }

    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    for n in nums {
        let squared = n.pow(2);
        println!("{n}: {squared}");
    }

    let names = ["skarekroe", "sanjana", "anisha"];

    // while index < names.len() {
    //     println!("{}", names[index]);
    //     index += 1;
    // }

    for name in names {
        println!("{name}");
    }
}
