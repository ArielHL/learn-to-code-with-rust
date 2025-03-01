fn main() {
    let teas: [String; 6] = [
        String::from("Earl Grey"),
        String::from("Green"),
        String::from("Chamomile"),
        String::from("Peppermint"),
        String::from("Lemon and Ginger"),
        String::from("English Breakfast"),
    ];

    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("More teas: {:?}", more_teas);
}
