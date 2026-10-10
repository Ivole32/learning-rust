use std::io::{self, Write};

fn main() {
    let mut input: String = String::new();

    print!("Type something: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");

    let input = input.trim();

    println!("You typed '{}'", input);
}
