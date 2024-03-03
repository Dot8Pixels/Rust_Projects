use rand::Rng;
use std::io;
use std::io::Write;

const GOAL_NUMBER: i32 = 21;

fn action_player_turn(hist_number: &mut Vec<i32>, player_turn: &mut bool) {
    println!("Player Play");

    *player_turn = false;

    let mut count_number: i32;

    loop {
        print!("How many numbers to push (1,2,3)? ");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        count_number = match input.trim().parse() {
            Ok(n) if (1..=3).contains(&n) => n,
            _ => {
                println!("Invalid count number. Please enter 1, 2, or 3.");
                continue;
            }
        };
        break;
    }

    loop {
        print!("{} input left: ", count_number);
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input_number = match input.trim().parse() {
            Ok(input_number) => input_number,
            Err(_) => {
                println!("Input Number Invalid i32 type.");
                0
            }
        };

        let last_value = hist_number.last().copied().unwrap_or(0);
        if last_value + 1 == input_number {
            hist_number.push(input_number);
            println!("{:?}", hist_number);
            if input_number == GOAL_NUMBER {
                break;
            }
            count_number -= 1;
        } else {
            println!("Invalid number sequence, please input number again.");
            println!("Previous Number: {:?}", hist_number);
            continue;
        }
        if count_number == 0 {
            break;
        }
    }
}

fn action_bot_turn(hist_number: &mut Vec<i32>, player_turn: &mut bool) {
    println!("Bot Play");

    *player_turn = true;

    let mut rng = rand::thread_rng();
    let count_number = rng.gen_range(1..3);
    println!("Bot chose to put {} number", count_number);

    for _i in 1..=count_number {
        let last_number = hist_number.last().copied().unwrap_or(0);
        let push_number = last_number + 1;
        hist_number.push(push_number);
        println!("Bot push number {}", push_number);
        println!("{:?}", hist_number);
        if push_number == GOAL_NUMBER {
            break;
        }
    }
}

fn main() {
    'round: loop {
        let mut hist_number: Vec<i32> = Vec::new();

        let mut rng = rand::thread_rng();
        let mut player_turn: bool = rng.gen();

        match player_turn {
            true => println!("Player Start First"),
            false => println!("Bot Start First"),
        }

        'play: loop {
            if hist_number.len() >= GOAL_NUMBER as usize {
                println!("Game Over!");
                match player_turn {
                    true => println!("You Win."),
                    false => println!("You lose."),
                }
                println!("========================================================");
                break 'play;
            }
            if player_turn {
                action_player_turn(&mut hist_number, &mut player_turn);
            } else {
                action_bot_turn(&mut hist_number, &mut player_turn);
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
