use crate::garden::veggies::Asparagus;

//the "pub mod garden" tells the compiler to include code it finds in src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {
        shape: String::from("long"),
        texture: String::from("crusty"),
    };
    println!("I'm growing {:?}", plant);
}
