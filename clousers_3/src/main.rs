fn main() {
    let mut game_console = String::from("PlayStation");
    let mut delete_character = String::new();

    let closure = |character: char| {
        let is_not_a = character != 'a';
        if is_not_a {
            true
        } else {
            delete_character.push(character);
            false
        }
    };

    game_console.retain(closure);
    println!("Game Console: {}", game_console);
    println!("Delete Character: {}", delete_character);
}
