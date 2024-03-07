use std::io;

struct LCG {
    m: u64, // Modulus
    a: u64, // Multiplier
    c: u64, // Increment
    s: u64, // Seed (Start Value)
}

// Implement https://en.wikipedia.org/wiki/Linear_congruential_generator
impl LCG {
    // Initialize the LCG with parameters and a seed
    fn new(m: u64, a: u64, c: u64, s: u64) -> Self {
        LCG { m, a, c, s }
    }

    // Generate the next random number
    fn next(&mut self) -> u64 {
        self.s = ((self.a * self.s) + self.c) % self.m;
        self.s
    }
}

fn main() {
    // Ask user for seed
    println!("Enter the seed for the random number generator:");
    let mut seed_input = String::new();
    io::stdin()
        .read_line(&mut seed_input)
        .expect("Failed to read line");
    let seed: u64 = seed_input.trim().parse().expect("Please enter a valid number");

    // Create lcg object with initial parameters and user-provided seed
    // Initial paramters from wiki chart https://en.wikipedia.org/wiki/Linear_congruential_generator
    let mut lcg = LCG::new(2u64.pow(32), 1664525, 1013904223, seed);

    // Generate a random number between 0 and 9
    let random_number = lcg.next() % 10;
    println!("Random number generated: {}", random_number);

    // Ask the user to guess the number
    println!("Guess the number! 0-9");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u64 = guess.trim().parse().expect("Please enter a valid number");

    // Compare the guess with the generated number
    if guess == random_number {
        println!("You win!");
    } else {
        println!("Fail, the number was {}.", random_number);
    }
}
