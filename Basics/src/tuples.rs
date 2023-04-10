// Tuples group tofwther a maximum of 12 values of different types

pub fn run() {
    let person: (&str, &str, i8) = ("Jacob", "Norther Ireland", 24);

    println!("{} is from {} and is {}.", person.0, person.1, person.2);
}