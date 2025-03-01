fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let my_iter = numbers.iter();

    for number in my_iter {
        println!("{}", number);
    }

    let mut cities = [
        "London".to_string(),
        "Lagos".to_string(),
        "Paris".to_string(),
        "New York".to_string(),
    ];
   
//    let iterator = cities.iter_mut();

   for city in & mut cities {
       city.push_str(", Nigeria");
   }

    println!("{:?}", cities);



    let mut school_grades = [34,56,76,89,76];

    for grade in& mut school_grades {
        *grade -= 2;
    }

    println!("{:?}", school_grades);
}
