mod board;

use board::Chessboard; 
use board::display_bit_board;

fn main() {
    let mut chessboard = Chessboard::new();

    let loc = 58;
    println!("{} {}", display_bit_board(chessboard.get_knight_move_mask(loc)), chessboard.get_knight_move_mask(loc));


    println!("{}", chessboard.display_board());
    /*
    // check if pawn exists
    display_bit_board(71776119061217280);
    chessboard.get_move_mask(55, true);
    chessboard.get_move_mask(15, false);

    // bring white pawn to front of black pieces
    chessboard.move_piece(51, 35, true);
    chessboard.move_piece(35, 27, true);
    
    // bring black pawn to front of white pieces
    chessboard.move_piece(8, 24, false);
    chessboard.move_piece(24, 32, false);

    chessboard.move_piece(49, 33, true);
    print!("{}", display_bit_board(chessboard.en_passant_square));
    let epb: bool =chessboard.move_piece(32, 41, false);
    chessboard.move_piece(12, 28, false);
    let epw =chessboard.move_piece(27, 20, true);

    println!("{} {} {}", chessboard.display_board(), epw, epb);

    print!("{}", chessboard._get_all_piece_mask());
    */
}
