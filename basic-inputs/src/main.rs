// Bring `io` (for stdin/stdout) and the `Write` trait (needed for `flush`) into scope
use std::io::{self, Write};

fn main() {
    // Create a new, empty string to store the user's input
    let mut input: String = String::new();

    // Print the prompt without a newline, then flush stdout
    // so the prompt shows up before the program waits for input
    print!("Type something: ");
    io::stdout().flush().unwrap();

    // Read a line from the terminal into `input` (includes the trailing newline)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");

    // Remove whitespace and the newline from both ends.
    // This shadows the old `input` with the trimmed version.
    let input = input.trim();

    // Print the input back inside single quotes
    println!("You typed '{}'", input);
}
