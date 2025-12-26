fn main() {
    println!("Hello, world!");
    println!("{}", "this is another line in rust");
    println!("this is the {}rd line", 3);

    println!("hello {0}, this is {1}", "skarekroe", "sanjana speaking");

    println!("base 2(binary): {:b}", 3237);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
}
