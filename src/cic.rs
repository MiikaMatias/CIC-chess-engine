// the implementation of the ai

// improvement ideas
// - sorting
// - nuanced heuristics (killer?)
// - parallel processing or whatever
// - handle ties
// - add opening book 
// - test some obvious evals

use crate::board::Chessboard;
use crate::board::display_bit_board;



const EXTENDED_CENTER:u64 = 66229406269440;
const PIECE_COUNT_EC: u64 = 16;
const DEPTH: u16 = 5; 

const PAWN_VAL:u32 = 100;
const KNIGHT_VAL:u32 = 300;
const BISHOP_VAL:u32 = 300;
const ROOK_VAL:u32 = 500;
const QUEEN_VAL:u32 = 900;

fn eval_extended_center(state: Chessboard, is_white_turn: bool) -> f64 {
    if is_white_turn {
        let pieces_in_center = state.get_white_pieces() & EXTENDED_CENTER;
        return f64::from(pieces_in_center.count_ones()) / PIECE_COUNT_EC as f64;    
    } else {
        let pieces_in_center = state.get_black_pieces() & EXTENDED_CENTER;
        return -f64::from(pieces_in_center.count_ones()) / PIECE_COUNT_EC as f64;    
    }
}

fn eval_win_threat(state: Chessboard, is_white_turn: bool) -> f64 {
    if state.check_win(is_white_turn) {
        return f64::INFINITY;
    } 
    if state.check_win(!is_white_turn) {
        return f64::NEG_INFINITY;
    }
    return 0.0;
}

fn eval_material(state: Chessboard, is_white_turn: bool) -> f64 {
    let white_knight_val = state.white_knight.count_ones() * KNIGHT_VAL;
    let white_bishop_val = state.white_bishop.count_ones() * BISHOP_VAL;
    let white_rook_val = state.white_rook.count_ones() * ROOK_VAL;
    let white_queen_val = state.white_queen.count_ones() * QUEEN_VAL;
    let white_pawn_val = state.white_pawn.count_ones() * PAWN_VAL; 
    let white_val = white_pawn_val + white_knight_val + white_bishop_val + white_rook_val + white_queen_val;

    let black_knight_val = state.black_knight.count_ones() * KNIGHT_VAL;
    let black_bishop_val = state.black_bishop.count_ones() * BISHOP_VAL;
    let black_rook_val = state.black_rook.count_ones() * ROOK_VAL;
    let black_queen_val = state.black_queen.count_ones() * QUEEN_VAL;
    let black_pawn_val = state.black_pawn.count_ones() * PAWN_VAL;
    let black_val = black_pawn_val + black_knight_val + black_bishop_val + black_rook_val + black_queen_val;

    if is_white_turn {
        return f64::from(white_val / black_val);
    } else {
        return -f64::from(black_val / white_val);
    }
}

// return board state evaluation as float between 0 and 1
fn primitive_heuristic_eval(state: Chessboard, is_white_turn: bool) -> f64 {
    let score_ext_center = eval_extended_center(state, is_white_turn); 
    let score_win_threat = eval_win_threat(state, is_white_turn);
    let score_material = eval_material(state, is_white_turn);


    return (score_ext_center + score_win_threat + score_material)/3.0;
}

fn order_by_heuristic(states: Vec<Chessboard>, is_white_turn: bool) -> Vec<Chessboard> {
    let mut evaluated_states = Vec::<(Chessboard, f64)>::new();
    for state in states {
        let eval = primitive_heuristic_eval(state, is_white_turn);
        evaluated_states.push((state, eval));
    }
    evaluated_states.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut ret_states = Vec::<Chessboard>::new();
    for state in evaluated_states {
        ret_states.push(state.0);
    }
    return ret_states;
}

fn max(a: f64, b : f64) -> f64 {
    if a > b {
        return a;
    }
    return b;
}

fn mini(a: f64, b : f64) -> f64 {
    if a < b {
        return a;
    }
    return b;
}

fn minimax(board: Chessboard, mut a: f64, mut b : f64, depth: u16, is_white_turn: bool) -> (Chessboard, f64) {
    if depth == 0 {
        return (board, primitive_heuristic_eval(board, is_white_turn));
    }

    let legal_moves = board._get_all_possible_moves(is_white_turn);
    let mut best_move: Chessboard = Chessboard::new();
    if is_white_turn {
        let mut current_eval = f64::NEG_INFINITY;
        for m in legal_moves {
            let (_, eval) = minimax(m, a, b,depth - 1, !is_white_turn);
            a = max(a, eval);
            if eval > current_eval {
                current_eval = eval;
                best_move = m;
            }
            if eval >= b {
                break;
            }
        }
        return (best_move, current_eval);
    } else {
        let mut current_eval = f64::INFINITY;
        for m in legal_moves {
            let (_, eval) = minimax(m, a, b, depth - 1, !is_white_turn);
            b = mini(b, eval);
            if eval < current_eval {
                current_eval = eval;
                best_move = m;
            }
            if eval <= a {
                break;
            }
        }
        return (best_move, current_eval);
    }
}

fn init_minimax(board: Chessboard, is_white_turn: bool, depth: u16) -> (Chessboard, f64) {
    let mut best_move = Chessboard::new();
    if is_white_turn {
        let mut eval = f64::NEG_INFINITY;
        let moves = order_by_heuristic(board._get_all_possible_moves(is_white_turn), 
        !is_white_turn);

        for m in moves {
            let (_, new_eval) = minimax(m, f64::NEG_INFINITY, f64::INFINITY, depth, !is_white_turn);
            if new_eval > eval {
                best_move = m;
                eval = new_eval;
            }
        }
        return (best_move, eval);

    } else {
        let mut eval = f64::INFINITY;
        let moves = order_by_heuristic(board._get_all_possible_moves(is_white_turn), 
        !is_white_turn);

        for m in moves {
            let (_, new_eval) = minimax(m, f64::NEG_INFINITY, f64::INFINITY, depth, !is_white_turn);
            if new_eval < eval {
                best_move = m;
                eval = new_eval;
            }
        }
        return (best_move, eval);
    }
    
}




use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn search_best_move(board: Chessboard, is_white_turn: bool) -> Chessboard {
    let start = std::time::Instant::now();
    let (board, eval) = init_minimax(board, is_white_turn, DEPTH);
    let end = start.elapsed().as_secs_f64();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("timetable.txt")
        .expect("Unable to open file");

    if let Err(e) = writeln!(
        file,
        "depth: {} \t time_taken: {}s\tevaluation: {}",
        DEPTH,
        end,
        eval
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }

    board
}

mod tests {
    use super::*;

    #[test]
    fn test_heuristic_eval_win() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(52, 36, true, true);
        chessboard._move_piece(12, 28, false, true);
        chessboard._move_piece(59, 31, true, true);
        chessboard._move_piece(8, 16, false, true);
        chessboard._move_piece(61, 34, true, true);
        chessboard._move_piece(16, 24, false, true);
        chessboard._move_piece(31, 13, true, true);
        assert_eq!(primitive_heuristic_eval(chessboard, true), f64::INFINITY);
        assert_eq!(primitive_heuristic_eval(chessboard, false), f64::NEG_INFINITY);
    }
    #[test]
    fn test_minimax_pick_checkmate_white() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(52, 36, true, true);
        chessboard._move_piece(12, 28, false, true);
        chessboard._move_piece(59, 31, true, true);
        chessboard._move_piece(8, 16, false, true);
        chessboard._move_piece(61, 34, true, true);
        chessboard._move_piece(16, 24, false, true);
        let (board_1, _) = init_minimax(chessboard, true, 1);
        let (board_2, _) = init_minimax(chessboard, true, 2);
        let (board_3, _) = init_minimax(chessboard, true, 3);
        chessboard._move_piece(31, 13, true, true);

        assert_eq!(board_1._display_board(), chessboard._display_board());
        assert_eq!(board_2._display_board(), chessboard._display_board());
        assert_eq!(board_3._display_board(), chessboard._display_board());
    }

    #[test]
    fn test_minimax_pick_checkmate_black() {
        let mut chessboard = Chessboard::new();

        chessboard._move_piece(52, 36, true, true); 
        chessboard._move_piece(60, 52, true, true); 
        chessboard._move_piece(52, 43, true, true); 
        chessboard._move_piece(43, 42, true, true); 
        chessboard._move_piece(42, 41, true, true); 
        chessboard._move_piece(41, 40, true, true); 
        chessboard._move_piece(11, 19, false, true); 
        chessboard._move_piece(3, 11, false, true); 
        chessboard._move_piece(11, 18, false, true); 
        chessboard._move_piece(18, 17, false, true); 
        chessboard._move_piece(12, 20, false, true);
        chessboard._move_piece(19, 27, false, true);  
        chessboard._move_piece(49, 33, true, true); 

        println!("{}", chessboard._display_board());
    
        let (board_1, _) = init_minimax(chessboard, false, 1);
    
        chessboard._move_piece(17, 33, false, true); 

        println!("{}\n{}", board_1._display_board(), chessboard._display_board());
    
        assert_eq!(board_1._display_board(), chessboard._display_board());
    }
}