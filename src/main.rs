use std::io;
const MATRIX_SIZE: usize = 10;
const HINT: &str = "Ꚛ";
const MISS: &str = "Ο";
const GRID: &str = "●";

// Function to read user's keyboard
fn user_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a valid string");
    input
}

// Function to validade that the input is a valid number
fn validade_input() -> usize {
    let mut input: String = String::new();
    loop {
        input = user_input();
        match input.trim().parse::<usize>() {
            Ok(_num) => break,
            Err(_) => println!("Invalid number. Please try again"),
        }
    }
    input.trim().parse::<usize>().unwrap()
}

// Function to create the game board with numeric index and 9x9 game area
fn construct_game_board() -> Vec<Vec<String>> {
    let mut matrix = vec![vec![String::new(); MATRIX_SIZE]; MATRIX_SIZE];
    let mut row_number: usize = 0;
    let mut column_number: usize = 1;
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            if i == 0 {
                matrix[i][j] = row_number.to_string();
                row_number += 1;
            } else {
                if j == 0 {
                    matrix[i][j] = column_number.to_string();
                    column_number += 1;
                } else {
                    matrix[i][j] = GRID.to_string();
                }
            }
        }
    }
    matrix // Returns game board matrix
}

// function to verify the attempt of the user hit a Ship
fn hint_game_board(row: usize, column: usize, ship: &Vec<Vec<String>>) -> bool {
    let result;
    if ship[row][column] == HINT {
        result = true;
    } else {
        result = false;
    }
    result
}

// funcion to print the matrix in a 2D grid
fn print_matrix(matrix: &Vec<Vec<String>>) {
    for row in matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

// Function to count strings in the matrix
fn count_matrix(matrix: &Vec<Vec<String>>) -> usize {
    let mut count = 0;
    for row in matrix {
        for val in row {
            if val == HINT {
                count += 1;
            }
        }
    }
    count
}

// Function to play the game
fn play_game() {
    clearscreen::clear().unwrap();
    let mut masked_game_board = construct_game_board();
    let mut ship_game_board = construct_game_board();
    let mut row: usize;
    let mut column: usize;
    print_matrix(&masked_game_board);

    // INSERIR LOGICA PARA CRIAR UMA STRING COM A POSIÇÃO RANDOM DOS NAVIOS
    ship_game_board[8][8] = HINT.to_string();

    loop {
        if count_matrix(&masked_game_board) == count_matrix(&ship_game_board) {
            println!("#################################################");
            println!("#### Congratulations, you hit all the ships! ####");
            println!("#################################################");
            println!();
            break;
        } else {
            loop {
                println!("Type the row: ");
                row = validade_input();
                match row {
                    1..MATRIX_SIZE => break,
                    ..1 | MATRIX_SIZE.. => println!("Invalid row, try again."),
                }
            }
            loop {
                println!("Type the column: ");
                column = validade_input();
                match column {
                    1..MATRIX_SIZE => break,
                    ..1 | MATRIX_SIZE.. => println!("Invalid column, try again."),
                }
            }

            clearscreen::clear().unwrap();
            println!("Your attempt -> Row: {} / Column: {}", row, column);

            if hint_game_board(row, column, &ship_game_board) {
                masked_game_board[row][column] = HINT.to_string();
            } else {
                masked_game_board[row][column] = MISS.to_string();
            }
            print_matrix(&masked_game_board);
        }
    }
}

// Function to show info to the user
fn game_info() {
    clearscreen::clear().unwrap();
    println!("------------------------------------------------------------------------------");
    println!("Battlehip is a guessing game for one or more players.");
    println!("The player must choose a row and column to try to hit the ships");
    println!("If there is no ship in the position, one attempt will be deducted");
    println!("The player can miss up to 10 attempts");
    println!("------------------------------------------------------------------------------");
}

fn main() {
    println!("-------------------------------------------");
    println!("  Welcome to the Battleship Game in Rust");
    println!("Personal challenge to test language skills");
    println!("   Developed by Matheus Eduardo Aver");
    println!("        matheusaver@gmail.com");
    println!("-------------------------------------------");
    // Show the options until a valid option is chosen
    loop {
        println!("Choose the option you want:");
        println!("1 - Play");
        println!("2 - Info");
        println!("3 - Exit");
        let game_option = validade_input();
        match game_option {
            1 => {
                play_game();
            }
            2 => {
                game_info();
                continue;
            }
            3 => break,
            _ => {
                clearscreen::clear().unwrap();
                println!("Invalid Option, choose another and try again.");
            }
        }
    }
}
