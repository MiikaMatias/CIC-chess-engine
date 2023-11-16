mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let cp = Chessboard::new();

    println!("{}", cp.display_board());

    // check if pawn exists
    println!("{}", display_bit_board(71776119061217280));

    println!("{}", display_bit_board(cp.get_pawn_attack_mask(9, false)));


    println!("{}", cp.get_pawn_move_mask(55, true));
    println!("{}", cp.get_pawn_move_mask(15, false));
}
