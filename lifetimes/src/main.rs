


fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
     &items[..2]
    
}
fn main() {
    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    let two_elements = select_first_two_elements(&items);
    println!("{:?}", two_elements);

    {
        let coffees = vec![
            "Americano".to_string(),
            "Cappuccino".to_string(),
            "Espresso".to_string(),
        ];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{:?}", two_coffees);
        println!("{:?}", two_elements);
    }

}
