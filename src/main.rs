use std::io; // Import standard input/output library
use rand::Rng; // Import random number generator
use std::cmp::Ordering; // Import the Ordering enum

fn main() {
    // Print a welcome message
    println!("Welcome to the guessing game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100); // Generates a number from 1 to 100 (inclusive)

    loop {
        // Prompt the user for their guess
        println!("Please input your guess:");

        let mut guess = String::new(); // Create a mutable String to hold user input

        // Read user input
        io::stdin()
            .read_line(&mut guess) // Read line into the guess variable
            .expect("Failed to read line"); // Handle potential errors

        // Parse the input to a number
        let guess: u32 = match guess.trim().parse() { // Trim whitespace and parse the guess
            Ok(num) => num, // If successful, store the number
            Err(_) => {
                println!("Please enter a valid number."); // Handle parsing errors
                continue; // Skip to the next iteration of the loop
            }
        };

        // Print the user's guess
        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // Guess is too low
            Ordering::Greater => println!("Too big!"), // Guess is too high
            Ordering::Equal => {
                println!("You guessed it! The secret number was {}.", secret_number); // Guess is correct
                break; 
            }
        }
    }
}
