fn main() {
    let first = String::from("Skarekroe"); //L1
    let first_clone = first.clone();
    let full = add_suffix(first_clone); //L4
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    //L2
    name.push_str(" Jr."); //L3
    name
}
