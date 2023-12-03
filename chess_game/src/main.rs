mod board;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;

fn main() {
    let mut chessboard = Chessboard::new();


    println!("{}", display_bit_board(1157442765409226768));
    chessboard.move_piece(8, 24, false);
    chessboard.move_piece(0, 16, false);
    chessboard.move_piece(16, 23, false);
    chessboard.move_piece(23, 55, false);

    print!("{} {}", display_bit_board(chessboard.get_rook_move_mask(47, true)), chessboard.get_rook_move_mask(47, true));

    println!("{}", chessboard.display_board());
}
