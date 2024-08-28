// the implementation of the ai

// improvement ideas

// first optimise search
// then finish board

// - magic bitboards
// - sorting MVV-LVA
// - dynamic programming
// - nuanced heuristics (killer?)
// - parallel processing or whatever
// - handle ties
// - add opening book 
// - test some obvious evals
// - add counting so that each move computes material in board.rs

use crate::board::Chessboard;
use crate::graphics::display_bit_board;
use crate::config::*;
use std::sync::LazyLock;

// shamelessly stolen from: https://rustic-chess.org/search/ordering/mvv_lva.html


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

fn eval_mvv_lva(state: Chessboard) -> f64 {
    return MVV_LVA_TABLE[state.last_captured as usize][state.last_capturee as usize];
}

fn eval_material(state: Chessboard, is_white_turn: bool) -> f64 {
    let white_val = state.white_knight.count_ones() * KNIGHT_VAL 
    + state.white_bishop.count_ones() * BISHOP_VAL 
    + state.white_rook.count_ones() * ROOK_VAL 
    + state.white_queen.count_ones() * QUEEN_VAL 
    + state.white_pawn.count_ones() * PAWN_VAL; 

    let black_val = state.black_knight.count_ones() * KNIGHT_VAL
    + state.black_bishop.count_ones() * BISHOP_VAL
    + state.black_rook.count_ones() * ROOK_VAL
    + state.black_queen.count_ones() * QUEEN_VAL
    + state.black_pawn.count_ones() * PAWN_VAL;

    if is_white_turn {
        return f64::from(white_val / black_val);
    } else {
        return -f64::from(black_val / white_val);
    }
}

// return board state evaluation
fn primitive_heuristic_eval(state: Chessboard, is_white_turn: bool) -> f64 {
    return  (eval_extended_center(state, is_white_turn)
    + eval_win_threat(state, is_white_turn)
    + eval_material(state, is_white_turn)
    + eval_mvv_lva(state)) / 4.0;
}

fn order_by_mvv_lva(states: Vec<Chessboard>) -> Vec<Chessboard> {
    let mut evaluated_states = states.iter().map(|s| (s, eval_mvv_lva(*s))).collect::<Vec<_>>();

    evaluated_states.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let result: Vec<_> = evaluated_states.into_iter().map(|(first, _)| *first).collect();
    return result;
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

fn minimax(board: Chessboard, mut a: f64, mut b : f64, depth: u16, is_white_turn: bool) -> f64 {
    if depth == 0 {
        return primitive_heuristic_eval(board, is_white_turn);
    }
    let legal_moves = (board._get_all_possible_moves(is_white_turn));

    if is_white_turn {
        let mut current_eval = f64::NEG_INFINITY;
        for m in legal_moves {
            let eval = minimax(m, a, b,depth - 1, !is_white_turn);
            a = max(a, eval);
            if eval > current_eval {
                current_eval = eval;
            }
            if eval >= b {
                break;
            }
        }
        return current_eval;
    } else {
        let mut current_eval = f64::INFINITY;
        for m in legal_moves {
            let eval = minimax(m, a, b, depth - 1, !is_white_turn);
            b = mini(b, eval);
            if eval < current_eval {
                current_eval = eval;
            }
            if eval <= a {
                break;
            }
        }
        return current_eval;
    }
}

fn init_minimax(board: Chessboard, is_white_turn: bool, depth: u16) -> (Chessboard, f64) {
    let mut best_move: Option<Chessboard> = None;
    if is_white_turn {
        let mut eval = f64::NEG_INFINITY;
        let moves = order_by_mvv_lva(board._get_all_possible_moves(is_white_turn));

        for m in moves {
            let new_eval = minimax(m, f64::NEG_INFINITY, f64::INFINITY, depth, !is_white_turn);
            if new_eval > eval {
                best_move = Some(m);
                eval = new_eval;
            }
        }
        return (best_move.unwrap(), eval);

    } else {
        let mut eval = f64::INFINITY;
        let moves = order_by_mvv_lva(board._get_all_possible_moves(is_white_turn));

        for m in moves {
            let new_eval = minimax(m, f64::NEG_INFINITY, f64::INFINITY, depth, !is_white_turn);
            if new_eval < eval {
                best_move = Some(m);
                eval = new_eval;
            }
        }
        return (best_move.unwrap(), eval);
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
        .open("reports/timetable.txt")
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
    use crate::precomps;
    use crate::graphics::display_board;

    use super::*;

    static PRECOMPS: LazyLock<precomps::Precomps> = LazyLock::new(|| precomps::Precomps::new());

    #[test]
    fn test_heuristic_eval_win() {
        let precomps: &precomps::Precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
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
        let precomps: &precomps::Precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
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

        assert_eq!(display_board(&board_1), display_board(&chessboard));
        assert_eq!(display_board(&board_2), display_board(&chessboard));
        assert_eq!(display_board(&board_3), display_board(&chessboard));
    }

    #[test]
    fn test_minimax_pick_checkmate_black() {
        let precomps: &precomps::Precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);

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
        chessboard._move_piece(8, 24, false, true);  
        chessboard._move_piece(49, 33, true, true); 
    
        let (board_1, _) = init_minimax(chessboard, false, 1);
        let (board_2, _) = init_minimax(chessboard, false, 2);
        let (board_3, _) = init_minimax(chessboard, false, 3);

        chessboard._move_piece(17, 33, false, true); 
        assert_eq!(display_board(&board_1), display_board(&chessboard));
        assert_eq!(display_board(&board_2), display_board(&chessboard));
        assert_eq!(display_board(&board_3), display_board(&chessboard));
    }
}