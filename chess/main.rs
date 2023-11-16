mod board;

use board::Chessboard;

fn main() {
    let cp = Chessboard::new();
    println!("{}", cp.display_board());
}
