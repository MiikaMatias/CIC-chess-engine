mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let cp = Chessboard::new();

    println!("{}", display_bit_board(72340172838076673));
    println!("{}", display_bit_board(9259542123273814144));

    println!("{}", cp.display_board());

    // check if pawn exists
    println!("{}", cp.get_pawn_attack_mask(54, true));
    println!("{}", cp.get_pawn_attack_mask(55, true));
}
