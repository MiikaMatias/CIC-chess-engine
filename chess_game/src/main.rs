mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let mut cp = Chessboard::new();

    // check if pawn exists
    display_bit_board(71776119061217280);
    cp.get_move_mask(55, true);
    cp.get_move_mask(15, false);


    // bring white pawn to front of black pieces
    cp.move_piece(51, 35, true);
    cp.move_piece(35, 27, true);
    cp.move_piece(27, 19, true);
    // bring black pawn to front of white pieces
    cp.move_piece(8, 24, false);
    cp.move_piece(24, 32, false);
    cp.move_piece(32, 40, false);

    //JUMP!
    let passed_white = cp.move_piece(48, 32, true);
    let passed_black = cp.move_piece(11, 27, false);
    println!("{} {} {}", cp.display_board(), passed_white, passed_black);

    print!("{}", cp._get_all_piece_mask());

    println!("{}",display_bit_board(262144));
}
