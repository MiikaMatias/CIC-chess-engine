mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let mut cp = Chessboard::new();

    // check if pawn exists
    display_bit_board(71776119061217280);
    cp.get_pawn_move_mask(55, true);
    cp.get_pawn_move_mask(15, false);

    print!("{}", display_bit_board(cp.white_pawn));
    println!("{}", cp.display_board());
    let passed = cp.move_piece(51, 35, true);
    println!("{} {}", cp.display_board(), passed);

    print!("{}", display_bit_board(cp.black_pawn));
    println!("{}", cp.display_board());
    let passed = cp.move_piece(11, 27, false);
    println!("{} {}", cp.display_board(), passed);

    print!("{}", display_bit_board(cp.white_pawn));
    println!("{}", cp.display_board());
    let passed = cp.move_piece(51, 27, true);
    println!("{} {}", cp.display_board(), passed);

    print!("{}", display_bit_board(cp.black_pawn));
    println!("{}", cp.display_board());
    let passed = cp.move_piece(12, 28, false);
    println!("{} {}", cp.display_board(), passed);


    print!("{}", display_bit_board(cp.white_pawn));
    println!("{}", cp.display_board());
    let passed = cp.move_piece(35, 28, true);
    println!("{} {}", cp.display_board(), passed);

}
