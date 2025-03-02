use std::collections::HashMap;

struct SupportStaff {
    day:String,
    name:String
}




fn main() {
    let earnings: [i32;4] = [4,7,10,15];
    let sum = earnings.iter().fold(0, |acc, x| acc + x);
    println!("Total earnings: {}", sum);


    // More Complex Example //////////////////////////////////////////////////////////
    let week = [
        SupportStaff {day: "Monday".to_string(), name: "John".to_string()},
        SupportStaff {day: "Tuesday".to_string(), name: "Jane".to_string()},
        SupportStaff {day: "Wednesday".to_string(), name: "Doe".to_string()},
        SupportStaff {day: "Thursday".to_string(), name: "Smith".to_string()}
    ];
    
    let staff = week.iter().fold(
        HashMap::new(),
        |mut acc,x| {
            acc.insert(&x.day, &x.name);
            acc
        }
    );
    println!("{:?}", staff);
}
