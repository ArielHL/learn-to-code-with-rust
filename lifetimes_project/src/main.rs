// defining functions 

fn double_the_length<T>(collection: &Vec<T>) -> usize{
    collection.len() * 2
}

fn last_two<T>(collection: &[T]) -> &[T] {
    let length = collection.len();
    let last_two = &collection[length - 2..];
    last_two
}

fn first_five<'a> (text: &'a str , announcement: &str) -> &'a str {
     println!("Announcement! {}", announcement);
     &text[..5]
    
}

fn find_string_that_has_content<'a> (first: &'a str, second: &'a str, target:&str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}



fn main() {
  let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec!["a", "b", "c", "d", "e"];
    let length_vec1 = double_the_length(&vec1);
    println!("The length of the vector is: {}", length_vec1);
    let length_vec2 = double_the_length(&vec2);
    println!("The length of the vector is: {}", length_vec2);


    let vec3 = vec![1, 2, 3, 4, 5];
    let last_two_vec3 = last_two(&vec3);
    println!("The last two elements of the vector are: {:?}", last_two_vec3);

    let text = String::from("Hello, world!");
    let announcement = String::from("This is an announcement");
    let first_five_chars = first_five(&text, &announcement);
    println!("The first five characters of the text are: {}", first_five_chars); 

    let first = String::from("Hello, world!");
    let second = String::from("Goodbye, world!");
    let target = String::from("Hello");
    
    let result = find_string_that_has_content(&first, &second, &target);
    println!("The result is: {}", result);

}
