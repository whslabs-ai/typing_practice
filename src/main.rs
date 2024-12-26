use std::io::{self, Write};
use std::time::Instant;
use rand::seq::SliceRandom;

fn main() {
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut rng = rand::thread_rng();

    println!("Typing Practice: Numbers on the First Line of the Keyboard");
    println!("Type the sequence of numbers as quickly and accurately as you can.");
    println!("Press Enter to start...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    loop {
        // Generate a random sequence of numbers
        let sequence: Vec<_> = numbers.choose_multiple(&mut rng, 10).cloned().collect();
        let sequence_str: String = sequence.iter().collect();

        println!("\nType this sequence: {}", sequence_str);

        // Start timing
        let start_time = Instant::now();

        // Get user input
        input.clear();
        print!("Your input: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let elapsed_time = start_time.elapsed();

        // Remove trailing newline
        let user_input = input.trim();

        // Check accuracy
        if user_input == sequence_str {
            println!("Correct! Time taken: {:.2?}", elapsed_time);
        } else {
            println!("Incorrect. The correct sequence was: {}", sequence_str);
        }

        println!("Do you want to try again? (y/n)");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Thank you for practicing!");
}
