struct User {
    name: String,
    email: String,
    age: u8,
}

fn main() {
    let user1 = User {
        name: String::from("sanskar"),
        email: String::from("sanskar@fake.com"),
        age: 27,
    };
    println!(
        "Name: {}, Email: {}, Age: {}",
        user1.name, user1.email, user1.age
    );

    let user2 = User {
        email: String::from("another_email@fake.com"),
        //COPIES THE PREVIOUS FIELDS FROM user1
        ..user1 //user1 is still in scope, but it is no longer valid to use because part of it was moved.
    };
    println!(
        "Name: {}, Email: {}, Age: {}",
        user2.name, user2.email, user2.age
    );

    let user3 = build_user(String::from("skarekroe"), 24);
    println!(
        "Name: {}, Email: {}, Age: {}",
        user3.name, user3.email, user3.age
    );

}

fn build_user(name: String, age: u8) -> User {
    User {
        name,
        email: String::from("skarekroe@fake.com"),
        age,
    }
}
