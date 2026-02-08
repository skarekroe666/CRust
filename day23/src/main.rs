fn main() {
    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    let mut v1 = vec![1, 2, 3, 4, 5];

    let third = &v1[2];
    println!("The third element is {third}");
    v1.pop();

    let fourth = v1.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element"),
    }

    for i in &mut v1 {
        *i += 10;
        println!("{i}");
    }

    println!("--------------------------------------------------------");

    let row = vec![
        Spreadsheet::Int(32),
        Spreadsheet::Float(3.2),
        Spreadsheet::Text(String::from("skarekroe")),
    ];

    // for cell in row {
    //     match cell {
    //         Spreadsheet::Int(i) => println!("{i}"),
    //         Spreadsheet::Float(f) => println!("{f}"),
    //         Spreadsheet::Text(s) => println!("{s}",),
    //     }
    // }

    for cell in row {
        if let Spreadsheet::Int(i) = cell {
            println!("{i}");
        }
    }
}

#[derive(Debug)]
enum Spreadsheet {
    Int(i32),
    Float(f64),
    Text(String),
}
