use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[3];

    let is_exist = File::open("hello.txt");
    let greeting_file = match is_exist {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new_file) => new_file,
                Err(e) => panic!("Error creating file: {e:?}"),
            },
            _ => panic!("Error creating file: {error:?}"),
        },
    };
    println!("{:?}", greeting_file);

    let another_file = File::open("hello.txt").expect("No such file");
    println!("{:?}", another_file);
}

fn read_username() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
