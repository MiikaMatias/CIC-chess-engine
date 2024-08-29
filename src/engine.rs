// the implementation of the ai

// improvement ideas

// - contain dynamic allocation; make-unmake instead of creating new board
// - eliminate branches
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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

// Define a static atomic counter to track the number of minimax calls.
static MINIMAX_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);


// if this shit starts making bizarre errors try googling zobrist hashing
// castling, en passant not supported
// Fix via Zobrist hashing |                                        pawn knight bishop rook queen king mover
static MINIMAX_HASH_TABLE: std::sync::LazyLock<std::sync::Mutex<HashMap<u64, i32>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

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
    eval_win_threat(state)
        .saturating_add(eval_extended_center(state))
        .saturating_add(eval_material(state))
}


fn order_by_mvv_lva(moves: Vec<Chessboard>) -> Vec<Chessboard> {
    // Your existing sorting logic...

    // Add logging to check the order of moves
    let mut ordered_moves = moves.iter().map(|s| (s, eval_mvv_lva(*s))).collect::<Vec<_>>();
    ordered_moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let result: Vec<_> = ordered_moves.into_iter().map(|(first, _)| *first).collect();

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

fn minimax(board: Chessboard, mut a: i32, mut b: i32, depth: i8, is_white_turn: bool) -> i32 {
    // Increment the minimax call counter.
    MINIMAX_CALL_COUNT.fetch_add(1, Ordering::Relaxed);

    if depth == 0 {
        return primitive_heuristic_eval(board);
    }

    let legal_moves = board.get_all_possible_moves(is_white_turn);
    if MINIMAX_HASH_TABLE.lock().unwrap().contains_key(&board.get_hash(depth as u64)) {
        return *MINIMAX_HASH_TABLE.lock().unwrap().get(&board.get_hash(depth as u64)).unwrap();
    }

    if is_white_turn {
        let mut current_eval = i32::MIN;
        for m in legal_moves {
            let eval = minimax(m, a, b, depth - 1, !is_white_turn);
            a = max(a, eval);
            if eval > current_eval {
                current_eval = eval;
            }
            if eval >= b {
                break;
            }
        }
        MINIMAX_HASH_TABLE.lock().unwrap().insert(board.get_hash(depth as u64), current_eval);
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
        MINIMAX_HASH_TABLE.lock().unwrap().insert(board.get_hash(depth as u64), current_eval);
        return current_eval;
    }
}

fn init_minimax(board: Chessboard, is_white_turn: bool, depth_max: i8) -> (Chessboard, i32) {
    let mut best_moves: Vec<Option<Chessboard>> = Vec::with_capacity(5);
    let mut eval = if is_white_turn { i32::MIN } else { i32::MAX };
    let mut moves = board.get_all_possible_moves(is_white_turn);
    moves = order_by_mvv_lva(moves);

    for depth in 1..=depth_max {
        if is_white_turn {
            for m in &moves {
                let new_eval = minimax(*m, i32::MIN, i32::MAX, depth, !is_white_turn);
                if new_eval > eval {
                    best_moves.push(Some(*m));
                    eval = new_eval;
                }
            }
            eval = eval;
        } else {            
            for m in &moves {
                let new_eval = minimax(*m, i32::MIN, i32::MAX, depth, !is_white_turn);
                if new_eval < eval {
                    best_moves.push(Some(*m));
                    eval = new_eval;
                }
            }
            eval = eval;
        }
        println!("depth: {}", depth);

        for mv in best_moves.iter() {
            moves.retain(|m| *m != *mv.as_ref().unwrap());
            moves.insert(0, mv.as_ref().unwrap().clone());
        }
    }
    return (best_moves[best_moves.len() -1].as_ref().unwrap().clone(), eval);
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

    let callcount = MINIMAX_CALL_COUNT.load(Ordering::Relaxed);

    if let Err(e) = writeln!(
        file,
        "depth: {} \t time_taken: {}s\tevaluation: {}  minimax_calls: {}",
        DEPTH,
        end,
        eval,
        callcount
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