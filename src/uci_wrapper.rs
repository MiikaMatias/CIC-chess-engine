use std::io::{self, BufRead};
use std::collections::HashMap;
use crate::board::Chessboard;
use crate::graphics::*;
use crate::engine::search_best_move;

pub fn uci_loop(mut board: Chessboard) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    println!("id name CIC");
    println!("id author Kontrakti");

    let mut is_white_turn: bool = true;

    loop {
        let input = match lines.next() {
            Some(line) => line,
            None => {
                eprintln!("Error: Unexpected end of input.");
                break;
            }
        };        
        let input_split: Vec<&str> = input.split_whitespace().collect();
        
        match input_split[0] {
            "uci" => {
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "getfen" => {
                println!("{}", generate_fen(&board));
            }
            "ucinewgame" => {
                board = Chessboard::new(board.precomps);
            }
            "position" => {
                if input_split.len() == 1 {
                    eprintln!("Error: Missing position option.");
                    continue;
                }
                if input_split[1] != "startpos" {
                    eprintln!("Error: Unknown position option.");
                    continue;
                }
                if input_split.len() == 2 {
                    apply_fen(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                    continue;
                }

                if input_split[2] == "moves" {
                    apply_fen(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                    for movestr in input_split.iter().skip(3) {
                        let move_to_play = translate_move(movestr);
                        board.move_piece(move_to_play[0], move_to_play[1]);
                    }
                    continue;
                }
                if input_split[2] == "fen" {
                    apply_fen(&mut board, &input_split[3..].join(" "));
                    continue;
                }
            }
            "go" => {
                /*
                let size_of_board_bytes = size_of::<Chessboard>();
                println!("The size of the board is {} B", size_of_board_bytes);
                */
                let new_board = search_best_move(board);
                println!("bestmove {}",get_uci_move(&board, &new_board, board.is_white));    
                apply_fen(&mut board, &generate_fen(&new_board));
            }
            "quit" => {
                break;
            }
            "br" => {
                println!("{}", display_board(&board));
            }
            "ma" => {
                if input_split.len() == 0 {
                    continue;
                }
                match input_split[1].parse::<u64>() {
                    Ok(pos) => {
                        let is_white = board.white_pieces & 1u64 << pos == 1u64 << pos;
                        let orig = board.is_white;
                        board.is_white = is_white;
                        let ma = board.get_all_moves_at_position(pos);
                        println!("{:?}", ma);
                        let mut ucima = Vec::<String>::new();
                        for m in ma {
                            ucima.push(format!("{}{}", num_to_coord(pos), num_to_coord(m)));
                        }
                        println!("{:?}", ucima);
                        board.is_white = orig;
                    },
                    Err(_) => {
                        println!("Error: Invalid position");
                    }
                }
            }
            "ep" => {
                println!("{}", display_bit_board(board.en_passant_square));
            }
            "flip" => {
                is_white_turn = !is_white_turn;
            }
            "gp" => {
                match input_split[1] {
                    "queen" => {
                        println!("{}", display_bit_board(board.queen));
                    }
                    _ => {
                        println!("Unknown command");
                    }
                }
            }
            "mv" => {
                let move_to_play = translate_move(&input_split[1]);
                board.move_piece(move_to_play[0], move_to_play[1]);
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

pub fn get_uci_move(cb_before: &Chessboard, cb_after: &Chessboard, is_white: bool) -> String {
    let mut from_square: u64 = 0;
    let mut to_square: u64 = 0;

    for pos in 0..64 {
        let mask = 1u64 << pos;
        let piece_before = if is_white {
            cb_before.white_pieces & mask != 0
        } else {
            cb_before.black_pieces & mask != 0
        };
        let piece_after = if is_white {
            cb_after.white_pieces & mask != 0
        } else {
            cb_after.black_pieces & mask != 0
        };

        if piece_before && !piece_after {
            from_square = pos;
        }
        if !piece_before && piece_after {
            to_square = pos;
        }
    }

    let promotion = if is_white && to_square < 8{
        cb_after.queen & (1u64 << to_square) != 0
    } else if !is_white && to_square >= 56 {
        cb_after.queen & (1u64 << to_square) != 0
    } else {
        false
    };

    let move_str = format!("{}{}", num_to_coord(from_square), num_to_coord(to_square));

    if promotion {
        return format!("{}q", move_str); // Assuming promotion to queen; adjust if needed
    }

    move_str
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

    for rank in 0..8 {
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

    #[test]
    fn test_get_uci_move() {
        let precomps = &PRECOMPS;
        let mut chessboard_1 = Chessboard::new(&precomps);
        apply_fen(&mut chessboard_1, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        let mut chessboard_2 = Chessboard::new(&precomps);
        apply_fen(&mut chessboard_2, "rnbqkbnr/ppppppp1/7p/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(get_uci_move(&chessboard_1, &chessboard_2, false), "h7h6");
    }
}