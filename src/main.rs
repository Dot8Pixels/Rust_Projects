use rand::Rng;
use std::io;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}

fn convert_to_game_move(choice: u8) -> Option<GameMove> {
    match choice {
        0 => Some(GameMove::Rock),
        1 => Some(GameMove::Paper),
        2 => Some(GameMove::Scissors),
        _ => None,
    }
}

fn get_player_move() -> u8 {
    print!("Choose your move (0 for Rock, 1 for Paper, 2 for Scissors): ");
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Defaulting to Rock.");
            0
        }
    }
}

fn get_bot_move() -> u8 {
    rand::thread_rng().gen_range(0..=2)
}

fn main() {
    loop {
        let player_choice = get_player_move();
        let bot_choice = get_bot_move();

        match (
            convert_to_game_move(player_choice),
            convert_to_game_move(bot_choice),
        ) {
            (Some(player_move), Some(bot_move)) => {
                println!("Player chose: {:?}", player_move);
                println!("Bot chose: {:?}", bot_move);

                match (player_move, bot_move) {
                    (GameMove::Rock, GameMove::Scissors)
                    | (GameMove::Paper, GameMove::Rock)
                    | (GameMove::Scissors, GameMove::Paper) => {
                        println!("You win!");
                    }
                    (GameMove::Rock, GameMove::Paper)
                    | (GameMove::Paper, GameMove::Scissors)
                    | (GameMove::Scissors, GameMove::Rock) => {
                        println!("Bot wins!");
                    }
                    _ => {
                        println!("It's a tie!");
                    }
                }
            }
            _ => {
                println!("Invalid choices. Please try again.");
            }
        }

        print!("Do you want to play again? (y/n): ");
        let mut play_again = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }
}
