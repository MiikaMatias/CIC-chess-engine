mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let mut cp = Chessboard::new();

    // check if pawn exists
    display_bit_board(71776119061217280);
    cp.get_move_mask(55, true);
    cp.get_move_mask(15, false);

    println!("{}",display_bit_board(262144));
}
