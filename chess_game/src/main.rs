mod board;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;

fn main() {
    let mut chessboard = Chessboard::new();
    println!("{}", display_bit_board(1157442765409226768));
    // Sample Chess Game

    /*
    // Initial Position
    chessboard._move_piece(6, 22, true);
    chessboard._move_piece(1, 18, false);
    chessboard._move_piece(57, 42, true);
    chessboard._move_piece(62, 45, false);
    chessboard._move_piece(5, 21, true);
    chessboard._move_piece(2, 20, false);

    // Developing Knights
    chessboard._move_piece(62, 61, true);
    chessboard._move_piece(1, 3, false);

    // Central Pawn Push
    chessboard._move_piece(48, 32, true);
    chessboard._move_piece(15, 31, false);

    // Castling
    chessboard._move_piece(60, 58, true);
    chessboard._move_piece(3, 5, false);
    chessboard._move_piece(58, 60, true);
    chessboard._move_piece(5, 3, false);

    // Queen's Gambit
    chessboard._move_piece(53, 37, true);
    chessboard._move_piece(12, 28, false);
    chessboard._move_piece(37, 28, true);
    chessboard._move_piece(4, 12, false);

    // Exchanging Pieces
    chessboard._move_piece(54, 45, true);
    chessboard._move_piece(13, 21, false);
    chessboard._move_piece(45, 36, true);
    chessboard._move_piece(21, 13, false);

    // King's Move
    chessboard._move_piece(60, 59, true);
    chessboard._move_piece(3, 4, false);
    chessboard._move_piece(59, 58, true);
    chessboard._move_piece(4, 3, false);

    // Bishop Development
    chessboard._move_piece(58, 56, true);
    chessboard._move_piece(5, 6, false);

    // Central Control
    chessboard._move_piece(51, 35, true);
    chessboard._move_piece(12, 27, false);
    chessboard._move_piece(35, 27, true);
    chessboard._move_piece(6, 12, false);

    // Rook Lift
    chessboard._move_piece(63, 61, true);
    chessboard._move_piece(0, 2, false);
    chessboard._move_piece(61, 60, true);
    chessboard._move_piece(2, 0, false);

    // Pawn Break
    chessboard._move_piece(52, 36, true);
    chessboard._move_piece(11, 27, false);
    chessboard._move_piece(36, 27, true);
    chessboard._move_piece(27, 11, false);

    // Knight Maneuver
    chessboard._move_piece(57, 49, true);
    chessboard._move_piece(4, 20, false);
    chessboard._move_piece(49, 50, true);
    chessboard._move_piece(20, 4, false);

    // Queen Centralization
    chessboard._move_piece(60, 56, true);
    chessboard._move_piece(3, 5, false);
    chessboard._move_piece(56, 52, true);
    chessboard._move_piece(5, 3, false);

    // Double Pawn Push
    chessboard._move_piece(50, 34, true);
    chessboard._move_piece(13, 29, false);

    // En passant
    chessboard._move_piece(34, 28, true);
    chessboard._move_piece(29, 36, false);

    // Checkmate
    chessboard._move_piece(59, 51, true);
    chessboard._move_piece(4, 11, false);
    chessboard._move_piece(51, 43, true);
    chessboard._move_piece(11, 4, false);
    chessboard._move_piece(43, 35, true);
     */
    chessboard._move_piece(51, 35, true, true);   // White moves
    chessboard._move_piece(59, 51, true, true);   // White moves
    chessboard._move_piece(51, 43, true, true);   // White moves
    chessboard._move_piece(43, 34, true, true);   // White moves
    chessboard._move_piece(34, 27, true, true);   // White moves
    let truval = chessboard._move_piece(27, 19, true, true);   // White moves
    chessboard._move_piece(6, 21, false, true);   // Black checks
    chessboard._move_piece(48, 40, true, true);   // White attempts to move pawns but can't

    println!("{} {}", chessboard.display_board(),(chessboard._get_all_piece_mask()));
    print!("{}", truval)
}