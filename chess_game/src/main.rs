mod board;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;

fn main() {
    let mut chessboard = Chessboard::new();


    println!("{}", display_bit_board(1157442765409226768));
    chessboard.move_piece(49, 41, true);
    chessboard.move_piece(58, 40, true);
    chessboard.move_piece(40, 12, true);
    chessboard.move_piece(5, 12, false);
    println!("{} {}", chessboard.display_board(),(chessboard._get_all_piece_mask()));
}
