/*
Primitive Types —
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Np. of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
 */

// As a statically types language, Rust needs to know the types of all variables at compile time. However the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let int32 = 1;
    
    // Default is "f64"
    let float64 = 2.5;

    // Add explicit type
    let int64: i64 = 45454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    
    
    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 < 5;

    // Chars
    let a1 = 'a';
    let face = '\u{1F600}';

    
    println!("{:?}", (int32, float64, int64, is_active, is_greater, a1, face));
    
}