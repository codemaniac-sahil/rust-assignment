use std::io;

fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.iter().collect()
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let reversed_string = reverse_string(input.trim());

    println!("Reversed string: {}", reversed_string);
}
