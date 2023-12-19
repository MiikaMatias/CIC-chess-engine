mod board;
mod uci_wrapper;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;
use uci_wrapper::uci_loop;


fn main() {
    let chessboard = Chessboard::new();

    uci_loop(chessboard);
}