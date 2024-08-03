mod board;
mod precomps;
mod uci_wrapper;
mod cic;

use board::Chessboard; 
#[allow(unused_imports)]
use board::display_bit_board;
use uci_wrapper::uci_loop;
use std::sync::LazyLock;

static PRECOMPS: LazyLock<precomps::Precomps> = LazyLock::new(|| precomps::Precomps::new());

fn main() {
    let chessboard = Chessboard::new(&PRECOMPS); // Pass a reference to precomps

    uci_loop(chessboard);
}