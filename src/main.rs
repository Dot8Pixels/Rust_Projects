use rand::seq::SliceRandom;
use rand::Rng;
use std::f32::consts::E;
use std::io::Write;
use std::{array, io};
const ROWS: usize = 4;
const COLS: usize = 4;

fn read_keyboard_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout.");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn display_table(board: [[i32; 4]; 4]) {
    for row in board {
        println!("{:?}", row);
    }
}

fn generate_new_random_position(mut board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut zero_position: Vec<[i32; 2]> = Vec::new();

    for (i, row) in board.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                zero_position.push([i as i32, j as i32]);
            }
        }
    }
    // println!("Empty Space: {:?}", zero_position);

    let mut rng = rand::thread_rng();

    if let Some([random_row, random_col]) = zero_position.choose(&mut rng) {
        // println!(
        //     "Randomly selected position: [{}, {}]",
        //     random_row, random_col
        // );

        // table[*random_row as usize][*random_col as usize] = 2;
        board[0][0] = 2;
    } else {
        // println!("The vector is empty.");
    }

    board
}

fn move_up(mut board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    board
}

fn move_down(mut board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    board
}

fn move_left(mut board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    board
}

fn move_right(mut board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut row_1 = board[0];

    for _ in (0..row_1.len()).rev() {
        row_1 = update_value(row_1);
    }

    board[0] = row_1;
    board
}

fn update_value(array_data: [i32; 4]) -> [i32; 4] {
    let data = array_data;

    let mut update_data = [0, 0, 0, 0];

    let last_value = data[data.len() - 1];
    let second_last_value = data[data.len() - 2];

    println!("{}", last_value);
    println!("{}", second_last_value);

    if last_value == second_last_value || last_value == 0 {
        for i in 0..data.len() {
            if i < 3 {
                update_data[i + 1] = data[i];
            }
        }
        return update_value(update_data);
    } else {
        println!("{:?}", update_data);
    }

    array_data
}

fn main() {
    let mut board = [[0; COLS]; ROWS];

    loop {
        board = generate_new_random_position(board);

        display_table(board);

        print!("Commands are as follows: ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let input = read_keyboard_input();
        let input_char = match input.trim().parse::<char>() {
            Ok(parse_char) => parse_char,
            Err(_) => {
                println!("Invalid input. Please enter a single character.");
                continue;
            }
        };
        let move_command = match input_char {
            'w' | 's' | 'a' | 'd' => input_char,
            _ => {
                println!("Wrong Command, please input command again.");
                continue;
            }
        };
        println!("{}", move_command);

        board = match move_command {
            'w' => move_up(board),
            's' => move_down(board),
            'a' => move_left(board),
            'd' => move_right(board),
            _ => continue,
        };
    }
}
