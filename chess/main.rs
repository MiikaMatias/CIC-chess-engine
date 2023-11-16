mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let cp = Chessboard::new();

    println!("{}", display_bit_board(cp.white_pawn));
    println!("{}", cp.display_board());
}
