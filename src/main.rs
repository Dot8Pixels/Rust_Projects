use rand::Rng;
use std::io;
use std::io::Write;

const HANGMAN_WORDS: [&str; 6] = [
    "rust",
    "programming",
    "developer",
    "hangman",
    "language",
    "coding",
];

fn main() {
    'round: loop {
        let mut life = 8;

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..HANGMAN_WORDS.len());
        let word = HANGMAN_WORDS[random_index];

        let mut guess_word: Vec<String> = vec!["_".to_string(); word.len()];
        println!("word contain {} characters: {:?}", word.len(), guess_word);

        for _ in 1..=word.len() as i32 {}

        loop {
            if life == 0 {
                println!("You lose!");
                println!("Game Over");
                break;
            }

            print!("Please input charactor: ");
            io::stdout().flush().expect("Failed to flush stdout.");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let guess_character = match input.trim().parse::<char>() {
                Ok(parsed_char) => {
                    println!("You entered a valid char: '{}'", parsed_char);
                    parsed_char
                }
                Err(_) => {
                    println!("Invalid input. Please enter a single character.");
                    continue;
                }
            };

            let indices: Vec<usize> = word
                .char_indices()
                .filter(|(_, c)| *c == guess_character)
                .map(|(index, _)| index)
                .collect();

            if indices.is_empty() {
                println!(
                    "The character '{}' is not in the word, please try again.",
                    guess_character
                );
                life -= 1;
                println!("{} guess left", life);
                continue;
            }
            println!(
                "The character '{}' is at indices {:?}",
                guess_character, indices
            );
            for index in indices {
                guess_word[index] = guess_character.to_string();
            }
            println!("{:?}", guess_word);
            let escape_char = "_";
            if guess_word.iter().any(|x| x == escape_char) {
                println!("Guess the next character.");
            } else {
                println!("You Win");
            }
        }

        'again: loop {
            print!("Wanna play again (y/n)?: ");
            let mut input = String::new();
            io::stdout().flush().expect("Failed to flush stdout.");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().to_lowercase().as_str() {
                "y" => {
                    println!("========================================================");
                    break 'again;
                }
                "n" => break 'round,
                _ => continue,
            };
        }
    }
}
