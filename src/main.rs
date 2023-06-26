use std::io::{self, Read};

fn main() {
    println!("Hello, this is a program that uses the Collatz conjecture.");
    println!("The conjecture asks whether repeating two simple arithmetic operations will eventually transform every positive integer into 1.");
    println!("Let's see, shall we?");

    loop {
        println!("Please enter a positive integer:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the input string into an integer
        let mut number: i64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };

        // Use the parsed integer
        println!("The number you entered is: {}", number);

        if number == 1 {
            println!("The number is already 1. No calculations needed.");
            break;
        }

        println!("Beginning the calculations.");

        let mut counter: i32 = 0;

        // Begin the main calcution loop
        while number != 1 {
            if number % 2 == 0 {
                number = number / 2;
                println!("The number is even, dividing by two.");
                println!("{:8} -->", number);
            } else {
                number = (number * 3) + 1;
                println!("The number is odd, tripling it and adding one.");
                println!("{:8} -->", number);
            }

            counter += 1;
        }

        println!("The calculations took {} steps.", counter);

        //loop to check the state of the program
        loop {
            println!("Please enter 'q' to quit or 'c' to continue:");

            let mut input_2 = String::new();
            io::stdin()
                .read_line(&mut input_2)
                .expect("Failed to read line");

            let trimmed_input = input_2.trim();

            match trimmed_input {
                "q" => {
                    println!("Closing the program.");
                    return;
                }
                "c" => {
                    println!("Continuing to the next iteration.");
                    break;
                }
                _ => {
                    println!("Invalid input. Please try again.");
                    println!("(Did you enter something other than 'q' or 'c' ?)");
                }
            }
        }
    }
}
