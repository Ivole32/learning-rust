use rand::prelude::*;

fn main() {
    let number = rand::rng().random_range(1..=100);

    println!("Your random number is {}", number)
}
