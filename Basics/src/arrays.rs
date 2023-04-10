// Arrays are fixed-length list where elements are the same data type.
pub fn run() {
    let nums: [i32; 5] = [0, 1, 2, 3, 4];
    let mut num_str = String::from("Nums: <");

    for num in nums {
        if nums.iter().position(|&n| n == num).unwrap() + 1 == nums.len() {
            num_str.push_str(&(num.to_string() + ">"));
        } else {
            num_str.push_str(&(num.to_string() + " "));
        }
    }

    println!("{num_str}");
}