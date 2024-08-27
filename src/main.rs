mod board;
mod precomps;
mod uci_wrapper;
mod engine;
mod precomps_knight_logic;
mod precomps_bishop_logic;
mod config;
mod precomps_rook_logic;
mod masks;
mod precomps_rook;
mod precomps_bishop;

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