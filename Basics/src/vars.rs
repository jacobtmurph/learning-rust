// Variables hold primitve data or references to data that are imutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Jacob";
    let mut age = 24;
    
    println!("My name is {name} and I am {age}.");

    // Define constants
    const ID: i32 = 001;
    println!("ID: {ID}");

    // Assign multiple vars
    let (my_name, my_age) = ("Jacob", 24);
    println!("{my_name} is {my_age}");
}