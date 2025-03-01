use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut chars_counts = HashMap::new();

    words.for_each(|word| {
        word.chars().for_each(|letter| {
            let count = chars_counts.entry(letter).or_insert(0);
            *count += 1;
        });
    });
    chars_counts
    // for word in words {
    //     for letter in word.chars() {
    //         let count = chars_counts.entry(letter).or_insert(0);
    //         *count += 1;
    //     }
    // }
    // chars_counts
}

fn main() {
    let text = "hello world hello";
    let word_count = count_words(text);
    println!("{:?}", word_count);
}
