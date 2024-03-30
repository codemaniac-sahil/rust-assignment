use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if input <= 1 {
        println!("{} is not prime.", input);
        return;
    }

    let mut is_prime = true;
    let mut divisor = 2;

    while divisor * divisor <= input {
        if input % divisor == 0 {
            is_prime = false;
            break;
        }
        divisor += 1;
    }

    if is_prime {
        println!("{} is prime.", input);
    } else {
        println!("{} is not prime.", input);
    }
}
