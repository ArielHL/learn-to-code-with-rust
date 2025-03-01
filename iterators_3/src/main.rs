fn main() {
    println!("--------------------------------------------------------------------------------");
    let numbers = vec![1, 2, 3, 4, 5];
    let my_iterator = numbers.iter();

    let squares = my_iterator.map(|num: &i32| num.pow(2));
    for square in squares {
        println!("{}", square);
    }

    // collect method
    println!("--------------------------------------------------------------------------------");
    let squares_collect: Vec<i32> = numbers.iter().map(|num: &i32| num.pow(2)).collect();
    println!("{:?}", squares_collect);

    // pipeline
    let names = [
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];


    let names_processed : Vec<usize>= names
                .iter()
                .map(|name: &String| name.to_uppercase())
                .map(|name: String| name.replace("A", "@@"))
                .map(|name: String| name.len())
                .collect();
    println!("{:?}", names_processed); 



    // Filter
    println!("--------------------------------------------------------------------------------");
    let numbers = [3,6,5,8,9,23,4,5,6,7,8,9,10];
    let even_numbers  :Vec<i32> = numbers
                        .into_iter()
                        .filter(|num: &i32| num % 2 == 0)
                        .collect();
    println!("{:?}", even_numbers);


}
