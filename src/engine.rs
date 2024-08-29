// the implementation of the ai

// improvement ideas

// first optimise search
// then finish board

// - cache
// - iterative 
// - sorting MVV-LVA
// - dynamic programming
// - nuanced heuristics (killer?)
// - parallel processing or whatever
// - handle ties
// - add opening book 
// - test some obvious evals
// - add counting so that each move computes material in board.rs

use crate::board::Chessboard;
use crate::config::*;
use std::sync::LazyLock;

// shamelessly stolen from: https://rustic-chess.org/search/ordering/mvv_lva.html


fn eval_extended_center(state: Chessboard) -> i32 {
    let w = state.get_white_pieces() & EXTENDED_CENTER;
    let b = state.get_black_pieces() & EXTENDED_CENTER;
    return (w.count_ones()) as i32  -((b.count_ones()) as i32);    
}

fn eval_win_threat(state: Chessboard) -> i32 {
    if state.check_win(true) {
        return i32::MAX;
    } 
    if state.check_win(false) {
        return i32::MIN;
    }
    return 0;
}

fn eval_mvv_lva(state: Chessboard) -> f64 {
    return MVV_LVA_TABLE[state.last_captured as usize][state.last_capturee as usize];
}

fn eval_material(state: Chessboard) -> i32 {
    return (state.get_white_knights().count_ones() as i32 * KNIGHT_VAL 
    + state.get_white_bishops().count_ones() as i32 * BISHOP_VAL 
    + state.get_white_rooks().count_ones() as i32 * ROOK_VAL 
    + state.get_white_queens().count_ones() as i32 * QUEEN_VAL 
    + state.get_white_pawns().count_ones() as i32 * PAWN_VAL
    - state.get_black_knights().count_ones() as i32 * KNIGHT_VAL
    - state.get_black_bishops().count_ones() as i32 * BISHOP_VAL
    - state.get_black_rooks().count_ones() as i32 * ROOK_VAL
    - state.get_black_queens().count_ones() as i32 * QUEEN_VAL
    - state.get_black_pawns().count_ones() as i32 * PAWN_VAL) as i32;
}

fn primitive_heuristic_eval(state: Chessboard) -> i32 {
    let win = eval_win_threat(state);
    if win != 0 {
        return win;
    }
    return  (eval_extended_center(state) + eval_material(state))
}

fn order_by_mvv_lva(states: Vec<Chessboard>) -> Vec<Chessboard> {
    let mut evaluated_states = states.iter().map(|s| (s, eval_mvv_lva(*s))).collect::<Vec<_>>();

    evaluated_states.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let result: Vec<_> = evaluated_states.into_iter().map(|(first, _)| *first).collect();
    return result;
}


fn max(a: i32, b : i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}

fn mini(a: i32, b : i32) -> i32 {
    if a < b {
        return a;
    }
    return b;
}

fn minimax(board: Chessboard, mut a: i32, mut b : i32, depth: i8, is_white_turn: bool) -> i32 {
    if depth == 0 {
        return primitive_heuristic_eval(board);
    }

    let legal_moves = order_by_mvv_lva(board.get_all_possible_moves(is_white_turn));

    if is_white_turn {
        let mut current_eval = i32::MIN;
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
        let mut current_eval = i32::MAX;
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

fn init_minimax(board: Chessboard, is_white_turn: bool, depth: i8) -> (Chessboard, i32) {
    let mut best_move: Option<Chessboard> = None;
    if is_white_turn {
        let mut eval = i32::MIN;
        let moves = board.get_all_possible_moves(is_white_turn);

        for m in moves {
            let new_eval = minimax(m, i32::MIN, i32::MAX, depth, !is_white_turn);
            if new_eval > eval {
                best_move = Some(m);
                eval = new_eval;
            }
        }
        return (best_move.unwrap(), eval);

    } else {
        let mut eval = i32::MAX;
        let moves = board.get_all_possible_moves(is_white_turn);

        for m in moves {
            let new_eval = minimax(m, i32::MIN, i32::MAX, depth, !is_white_turn);
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
        chessboard.move_piece(52, 36, true);
        chessboard.move_piece(12, 28, false);
        chessboard.move_piece(59, 31, true);
        chessboard.move_piece(8, 16, false);
        chessboard.move_piece(61, 34, true);
        chessboard.move_piece(16, 24, false);
        chessboard.move_piece(31, 13, true);
        assert_eq!(primitive_heuristic_eval(chessboard), i32::MAX);
    }
    #[test]
    fn test_minimax_pick_checkmate_white() {
        let precomps: &precomps::Precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(52, 36, true);
        chessboard.move_piece(12, 28, false);
        chessboard.move_piece(59, 31, true);
        chessboard.move_piece(8, 16, false);
        chessboard.move_piece(61, 34, true);
        chessboard.move_piece(16, 24, false);
        let (board_1, _) = init_minimax(chessboard, true, 1);
        let (board_2, _) = init_minimax(chessboard, true, 2);
        let (board_3, _) = init_minimax(chessboard, true, 3);
        chessboard.move_piece(31, 13, true);

        assert_eq!(display_board(&board_1), display_board(&chessboard));
        assert_eq!(display_board(&board_2), display_board(&chessboard));
        assert_eq!(display_board(&board_3), display_board(&chessboard));
    }

    #[test]
    fn test_minimax_pick_checkmate_black() {
        let precomps: &precomps::Precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);

        chessboard.move_piece(52, 36, true); 
        chessboard.move_piece(60, 52, true); 
        chessboard.move_piece(52, 43, true); 
        chessboard.move_piece(43, 42, true); 
        chessboard.move_piece(42, 41, true); 
        chessboard.move_piece(41, 40, true); 
        chessboard.move_piece(11, 19, false); 
        chessboard.move_piece(3, 11, false); 
        chessboard.move_piece(11, 18, false); 
        chessboard.move_piece(18, 17, false); 
        chessboard.move_piece(12, 20, false);
        chessboard.move_piece(19, 27, false); 
        chessboard.move_piece(8, 24, false);  
        chessboard.move_piece(49, 33, true); 
    
        let (board_1, _) = init_minimax(chessboard, false, 1);
        let (board_2, _) = init_minimax(chessboard, false, 2);
        let (board_3, _) = init_minimax(chessboard, false, 3);

        chessboard.move_piece(17, 33, false); 
        assert_eq!(display_board(&board_1), display_board(&chessboard));
        assert_eq!(display_board(&board_2), display_board(&chessboard));
        assert_eq!(display_board(&board_3), display_board(&chessboard));
    }
}