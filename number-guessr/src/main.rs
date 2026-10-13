// Bring the most common items from the `rand` crate into scope (e.g. `random_range`)
use rand::prelude::*;
// Bring `io` (for stdin/stdout) and the `Write` trait (needed for `flush`) into scope
use std::io::{self, Write};

fn main() {
    // Generate the secret number between 1 and 100 (both included)
    let rnd_number = rand::rng().random_range(1..=100);

    // Keep asking for guesses until the player guesses correctly
    loop {
        // Create a new, empty string to store the player's input
        let mut input: String = String::new();

        // Print the prompt without a newline, then flush stdout
        // so the prompt shows up before the program waits for input
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        // Read a line from the terminal into `input` (includes the trailing newline)
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Remove whitespace and the newline, then convert the text into an i32.
        // If the input is not a valid number, show a message and ask again.
        let input: i32 = match input.trim().parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Skip the rest of this round and start the loop again
            }
        };

        // Compare the guess with the secret number and give a hint
        if rnd_number == input {
            println!("Correct! You guessed the number!");
            break; // Correct guess: leave the loop and end the game
        } else if rnd_number > input {
            println!("Too low! Try a higher number.");
        } else if rnd_number < input {
            println!("Too high! Try a lower number.");
        }
    }
}
