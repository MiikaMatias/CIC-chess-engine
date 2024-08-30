use std::io::{self, BufRead};
use std::collections::HashMap;
use std::mem::size_of;
use crate::board::Chessboard;
use crate::graphics::display_board;
use crate::engine::search_best_move;

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
                println!("option name OptionName type OptionType default OptionDefault");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {
                is_white_turn = true;
                board = Chessboard::new(board.precomps);
            }
            "position" => {
                if input_split[1] == "startpos" && input_split[2] == "moves" {
                    for movestr in input_split.iter().skip(3) {
                        let move_to_play = translate_move(movestr);

                        let piece_moved = board.move_piece(move_to_play[0], move_to_play[1], is_white_turn);
                        if !piece_moved {
                            println!("The move {} -> {} was not successful", move_to_play[0], move_to_play[1]);
                            continue;
                        } else {
                            println!("moved from {} to {}", move_to_play[0], move_to_play[1]);
                            println!("{}", display_board(&board));
                        }
                        is_white_turn = !is_white_turn;
                    }
                }
            }
            "go" => {
                let size_of_board_bytes = size_of::<Chessboard>();
                println!("The size of the board is {} B", size_of_board_bytes);
                println!("{}", display_board(&board));
                board = search_best_move(board, is_white_turn);
                println!("{}", display_board(&board));    
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



pub fn translate_move(uci: &str) -> [u64; 2] {
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

    if uci.len() == 2 {
        let file_from = *nummap.get(&uci.chars().next().unwrap()).unwrap_or(&10000);
        let rank_from = uci.chars().nth(1).unwrap().to_digit(10).unwrap() as u64;
    
        let from = (7-(rank_from-1))*8 + (file_from);
        return [from, 100000]    
    } else if uci.len() == 4 {
        let file_from = *nummap.get(&uci.chars().next().unwrap()).unwrap_or(&10000);
        let rank_from = uci.chars().nth(1).unwrap().to_digit(10).unwrap() as u64;
        let file_to = *nummap.get(&uci.chars().nth(2).unwrap()).unwrap_or(&10000);
        let rank_to = uci.chars().nth(3).unwrap().to_digit(10).unwrap() as u64;
    
        let from = (7-(rank_from-1))*8 + (file_from);
        let to = (7-(rank_to-1))*8 + (file_to);
        return [from, to]    
    }
    return [10000, 100000];
}

pub fn num_to_coord(num: u64) -> String {
    let file = num % 8;
    let rank = 7 - (num / 8);
    return format!("{}{}", ('a' as u8 + file as u8 ) as char, rank+1);
}

pub fn apply_fen(cb: &mut Chessboard, fen: &str) {
        
    let parts: Vec<&str> = fen.split(" ").collect();
    if parts.len() != 6 {
        panic!("FEN must have 6 parts");
    }
    cb.pawn=0;
    cb.rook=0;
    cb.knight=0;
    cb.bishop=0;
    cb.queen=0;
    cb.king=0;
    cb.white_pieces=0;
    cb.black_pieces=0;
    cb.en_passant_square=0;
    cb.last_captured=0;
    cb.last_capturee=0;
    cb.is_white=true;

    let board = parts[0];
    let turn = parts[1];
    // let castling = parts[2];
    let en_passant_square = parts[3];
    //let half_moves = parts[4];
    //let full_moves = parts[5];

    let mut pos = 0;
    for c in board.chars() {
        match c {
            'p' => {
                cb.pawn |= 1u64 << pos;
                cb.black_pieces |= 1u64 << pos;
            },
            'n' => {
                cb.knight |= 1u64 << pos;
                cb.black_pieces |= 1u64 << pos;
            },
            'b' => {
                cb.bishop |= 1u64 << pos;
                cb.black_pieces |= 1u64 << pos;
            },
            'r' => {
                cb.rook |= 1u64 << pos;
                cb.black_pieces |= 1u64 << pos;
            },
            'q' => {
                cb.queen |= 1u64 << pos;
                cb.black_pieces |= 1u64 << pos;
            },
            'k' => {
                cb.king |= 1u64 << pos;
                cb.black_pieces |= 1u64 << pos;
            }
            'P' => {
                cb.pawn |= 1u64 << pos;
                cb.white_pieces |= 1u64 << pos;
            },
            'N' => {
                cb.knight |= 1u64 << pos;
                cb.white_pieces |= 1u64 << pos;
            },
            'B' => {
                cb.bishop |= 1u64 << pos;
                cb.white_pieces |= 1u64 << pos;
            },
            'R' => {
                cb.rook |= 1u64 << pos;
                cb.white_pieces |= 1u64 << pos;
            },
            'Q' => {
                cb.queen |= 1u64 << pos;
                cb.white_pieces |= 1u64 << pos;
            },
            'K' => {
                cb.king |= 1u64 << pos;
                cb.white_pieces |= 1u64 << pos;
            }
            '/' => continue,
            '1' => pos += 0,
            '2' => pos += 1,
            '3' => pos += 2,
            '4' => pos += 3,
            '5' => pos += 4,
            '6' => pos += 5,
            '7' => pos += 6,
            '8' => pos += 7,
            _ => panic!("Invalid FEN character: {}", c)
        }
        pos += 1;
    }

    if turn == "w" {
        cb.is_white = true;
    } else if turn == "b" {
        cb.is_white = false;
    } else {
        panic!("Invalid FEN turn: {}", turn);
    }

    if en_passant_square != "-" {
        cb.en_passant_square = crate::uci_wrapper::translate_move(en_passant_square)[0];
    }
}


pub fn generate_fen(cb: &Chessboard) -> String {
    let mut fen = String::new();
    let mut empty_count = 0;

    for rank in (0..8) {
        for file in 0..8 {
            let pos = rank * 8 + file;
            let piece = if (cb.pawn & (1u64 << pos)) != 0 {
                if (cb.white_pieces & (1u64 << pos)) != 0 { 'P' } else { 'p' }
            } else if (cb.knight & (1u64 << pos)) != 0 {
                if (cb.white_pieces & (1u64 << pos)) != 0 { 'N' } else { 'n' }
            } else if (cb.bishop & (1u64 << pos)) != 0 {
                if (cb.white_pieces & (1u64 << pos)) != 0 { 'B' } else { 'b' }
            } else if (cb.rook & (1u64 << pos)) != 0 {
                if (cb.white_pieces & (1u64 << pos)) != 0 { 'R' } else { 'r' }
            } else if (cb.queen & (1u64 << pos)) != 0 {
                if (cb.white_pieces & (1u64 << pos)) != 0 { 'Q' } else { 'q' }
            } else if (cb.king & (1u64 << pos)) != 0 {
                if (cb.white_pieces & (1u64 << pos)) != 0 { 'K' } else { 'k' }
            } else {
                empty_count += 1;
                continue;
            };

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
                empty_count = 0;
            }
            fen.push(piece);
        }

        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
            empty_count = 0;
        }
        
        if rank != 7 {
            fen.push('/');
        }
    }

    if cb.is_white {
        fen.push_str(" w ");
    } else {
        fen.push_str(" b ");
    }

    fen.push_str("- "); // FIX WITH CASTLE

    if cb.en_passant_square > 0 {
        let en_passant = num_to_coord(cb.en_passant_square);
        fen.push_str(&en_passant);
    } else {
        fen.push('-');
    }

    fen.push_str(" 0 1");

    fen
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use crate::precomps;
    static PRECOMPS: LazyLock<precomps::Precomps> = LazyLock::new(|| precomps::Precomps::new());

    #[test]
    fn test_apply_fen() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        crate::uci_wrapper::apply_fen(&mut chessboard, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(chessboard.get_all_pieces(), 18441959067825012735);
        assert_eq!(chessboard.en_passant_square, 44);
        crate::uci_wrapper::apply_fen(&mut chessboard, "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 1 2");
        assert_eq!(chessboard.get_all_pieces(), 13830308233836821503);
    }

    #[test]
    fn test_generate_fen() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        assert_eq!(generate_fen(&chessboard), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1");

        // play a standard opening 
        apply_fen(&mut chessboard, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(generate_fen(&chessboard), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b - e3 0 1");
    }
}