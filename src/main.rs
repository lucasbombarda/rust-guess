use rand::{thread_rng, Rng};
use std::io::{stdin, stdout, Write};

const MIN_VAL: usize = 0;
const MAX_VAL: usize = 10;
const MAX_ATTEMPTS: usize = 5;

fn generate_random_number() -> usize {
    thread_rng().gen_range(MIN_VAL..=MAX_VAL)
}

fn main() {
    loop {
        let random_number = generate_random_number();
        let mut current_attempt = 0;

        println!(
            "A number between {} and {} was generated.",
            MIN_VAL, MAX_VAL
        );

        while current_attempt < MAX_ATTEMPTS {
            println!(
                "[ATTEMPT {}/{}] Give it a guess!",
                current_attempt + 1,
                MAX_ATTEMPTS
            );

            stdout().flush().unwrap();

            let mut guess = String::new();
            stdin()
                .read_line(&mut guess)
                .expect("Error: Could not read your guess");

            match guess.trim().parse::<usize>() {
                Ok(parsed_guess) => {
                    if parsed_guess < random_number {
                        println!("No! The answer is ᵇᶦᵍᵍᵉʳ!");
                        current_attempt += 1;
                    } else if parsed_guess > random_number {
                        println!("No! The answer is ₛₘₐₗₗₑᵣ!");
                        current_attempt += 1;
                    } else {
                        println!("\nYes! It was {parsed_guess}! You won!");
                        break;
                    }
                }
                Err(_) => println!("Insert only numbers!"),
            }
        }

        if current_attempt == MAX_ATTEMPTS {
            println!("\nSorry you ran out of attempts! The answer was {random_number}.");
        }

        println!("Try again? [Y/n]");
        stdout().flush().unwrap();

        let mut yorn = String::new();
        stdin()
            .read_line(&mut yorn)
            .expect("Error: Could not read your answer!");

        match yorn.trim().to_uppercase().as_str() {
            "N" | "NO" => {
                println!("Ok, bye!");
                break;
            }
            _ => {
                println!();
                continue;
            }
        }
    }
}
