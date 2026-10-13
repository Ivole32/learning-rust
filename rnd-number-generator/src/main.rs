// Bring the most common items from the `rand` crate into scope (e.g. `random_range`)
use rand::prelude::*;

fn main() {
    // Generate a random number between 1 and 100 (both included)
    let number = rand::rng().random_range(1..=100);

    // Print the generated number
    println!("Your random number is {}", number)
}
