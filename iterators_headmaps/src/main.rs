use std::collections::HashMap;

fn main() {
    let mut todos = HashMap::new();
    todos.insert("Buy milk", false);
    todos.insert("Buy bread", true);
    todos.insert("Buy butter", false);

    for (todo, completion_status) in &todos {
        if *completion_status {
            println!("{} is done", todo);
        } else {
            println!("{} is not done", todo);
        }
    }

    println!("Number of todos: {}", todos.len());

}
