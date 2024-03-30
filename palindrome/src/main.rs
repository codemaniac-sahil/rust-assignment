use std::io;

fn is_palindrome(input: &str) -> bool {
    let reversed: String = input.chars().rev().collect();
    input == reversed
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    if is_palindrome(&input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
