mod board;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;

fn main() {
    let mut chessboard = Chessboard::new();


    println!("{}", display_bit_board(1157442765409226768));
    chessboard.move_piece(51, 35, true);
    chessboard.move_piece(60, 24, true);
    chessboard.move_piece(24, 10, true);
    chessboard.move_piece(10, 1, true);
    chessboard.move_piece(1, 2, true);
    chessboard.move_piece(2, 3, true);
    chessboard.move_piece(4, 3, false);
    println!("{}", display_bit_board(chessboard.get_queen_move_mask(24, true)));
    println!("{} {}", chessboard.display_board(),(chessboard._get_all_piece_mask()));
}