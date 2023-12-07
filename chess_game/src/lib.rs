mod board;
mod uci_wrapper;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;
use uci_wrapper::_uci_loop;


fn main() {
    let mut chessboard = Chessboard::new();

    _uci_loop(chessboard);
}