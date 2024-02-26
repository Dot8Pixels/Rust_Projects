use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn update_previous_number(previous_guess_number: &mut Vec<i32>, guess_number: i32) {
    previous_guess_number.push(guess_number);

    let previous_number_string = previous_guess_number
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Previous Guess Number: {:?}", previous_number_string);
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Welcome to Number Guessing Game!!!\nRule: Guess the number between 0 to 100.");

    'game: loop {
        let mut previous_guess_number: Vec<i32> = Vec::new();

        let random_number: i32 = rng.gen_range(1..=100);

        'guess: loop {
            let mut input = String::new();

            print!("Enter Guess Number: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let guess_number: i32 = match input.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Invalid input. Please enter a valid integer.");
                    return;
                }
            };

            match guess_number.cmp(&random_number) {
                Ordering::Less => {
                    println!("The number is too low.");
                    update_previous_number(&mut previous_guess_number, guess_number)
                }
                Ordering::Greater => {
                    println!("The number is too high.");
                    update_previous_number(&mut previous_guess_number, guess_number)
                }
                Ordering::Equal => {
                    println!("Congratulation!, you win the game.");
                    break 'guess;
                }
            }
        }

        print!("Do you want to play again? (y/n)");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            break 'game;
        }
    }
}
