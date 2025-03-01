use std::collections::HashMap;

struct SupportStaff {
    day:String,
    name:String
}




fn main() {
    let earnings: [i32;4] = [4,7,10,15];
    let sum = earnings.into_iter().fold(0, |acc, x| acc + x);
    println!("Total earnings: {}", sum);
}
