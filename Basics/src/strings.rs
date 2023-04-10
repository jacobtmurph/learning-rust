// Primitive str = Immutable fixed-length string somewhere in memory.
// String = Growable, heap-allocated data structure — for when you need to modify or own string data.

pub fn run() {
    let mut hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    // Push string
    hello.push_str(" There");

    // Push char
    hello.push('!');

    // Bytye capacity of a string
    println!("Capacity: {}", hello.capacity());

    // Check if string is empty
    println!("Is Empty? {}", hello.is_empty());

    // Check is string contains a sub-string
    println!("Contains 'there'? {}", hello
                                     .to_lowercase()
                                     .contains("there"));
    // Replace substring with new value
    println!("Replace: {}", hello.replace("There", "World"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{word}");
    }
    
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{s}");

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    
    println!("{}", hello);
}