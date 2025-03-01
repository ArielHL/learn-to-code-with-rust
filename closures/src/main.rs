fn main() {
    // Closures are anonymous functions that can capture their environment
    // Closures from Inmutable References (FN)
    let multiplier = 5;
    let multiply_by = |value: i32| value * multiplier;

    let result = multiply_by(10);
    println!("Result: {}", result);

    let numbers = vec![1, 2, 3, 4, 5];

    let print_numbers = || println!("Numbers: {:?}", numbers);
    print_numbers();

    // Closures from Mutable References (FNMut)
    let mut numbers_mut = vec![1, 2, 3, 4, 5];

    let mut add_number = |value| numbers_mut.push(value);
    add_number(100);
    add_number(200);
    add_number(300);
    println!("Numbers: {:?}", numbers_mut);

    // Closures that take ownership of the environment (FNOnce)
    let number = 5;
    let capture_number = || number;

    println!("Number: {}", capture_number());

    let first_name = String::from("John");
    let capture_string = || first_name;
    let owner = capture_string();




}
