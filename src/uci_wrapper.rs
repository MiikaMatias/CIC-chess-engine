use std::io::{self, BufRead};
use std::collections::HashMap;
use crate::board::Chessboard;
use crate::cic::search_best_move;

pub fn uci_loop(mut board: Chessboard) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    println!("id name Magachess");
    println!("id author Kontrakti");
    println!("uciok");

    let mut is_white_turn = true;

    loop {
        let input = lines.next().unwrap();
        let input_split: Vec<&str> = input.split_whitespace().collect();
    
        match input_split[0] {
            "uci" => {
                // Send UCI options, if any
                println!("option name OptionName type OptionType default OptionDefault");
                println!("uciok");
            }
            "isready" => {
                // Respond to isready command
                println!("readyok");
            }
            "ucinewgame" => {
                // Clear the board and prepare for a new game
                is_white_turn = true;
            }
            "position" => {
                if input_split[1] == "startpos" && input_split[2] == "moves" {
                    for movestr in input_split.iter().skip(3) {
                        let move_to_play = translate_move(movestr);

                        let piece_moved = board._move_piece(move_to_play[0], move_to_play[1], is_white_turn, true);
                        if !piece_moved {
                            println!("The move {} -> {} was not successful", move_to_play[0], move_to_play[1]);
                            continue;
                        } else {
                            println!("moved from {} to {}", move_to_play[0], move_to_play[1]);
                            println!("{}", board._display_board());
                        }
                        is_white_turn = !is_white_turn;
                    }
                }
            }
            "go" => {
                board = search_best_move(board, is_white_turn);
                println!("{}", board._display_board());
            }
            "quit" => {
                break;
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}



fn translate_move(uci: &str) -> [u64; 2] {
    if uci.len() != 4 {
        return [10000, 100000]; // UCI move should have exactly 4 characters
    }

    let nummap: HashMap<char, u64> = vec![
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
    ]
    .into_iter()
    .collect();

    let file_from = *nummap.get(&uci.chars().next().unwrap()).unwrap_or(&10000);
    let rank_from = uci.chars().nth(1).unwrap().to_digit(10).unwrap() as u64;
    let file_to = *nummap.get(&uci.chars().nth(2).unwrap()).unwrap_or(&10000);
    let rank_to = uci.chars().nth(3).unwrap().to_digit(10).unwrap() as u64;

    let from = (7-(rank_from-1))*8 + (file_from);
    let to = (7-(rank_to-1))*8 + (file_to);
    [from, to]
}
