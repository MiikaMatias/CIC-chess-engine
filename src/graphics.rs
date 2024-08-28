use crate::Chessboard;

#[allow(dead_code)]
pub fn display_bit_board(board: u64) -> String {
    let rows = 8;
    let cols = 8;
    let mut board_string = String::new();

    for i in 0..rows {
        for j in 0..cols {
            let bit_position = i * cols + j;
            let bit_value = (board >> bit_position) & 1u64;

            let piece_char = if bit_value == 1 { '1' } else { '0' };
            board_string.push(piece_char);
        }
        board_string.push('\n');
    }

    board_string
}

pub fn display_board(board: &Chessboard) -> String {
    let rows = 8;
    let cols = 8;
    let mut board_string = String::new();
    board_string.push_str("    0 1 2 3 4 5 6 7\n");
    board_string.push_str("    ----------------\n");

    for i in 0..rows {
        board_string.push_str(&format!("{:2}| ", i*8));
        for j in 0..cols {
            let mut piece_char = 'e';

            if (board.black_pawn & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'p';
            } else if (board.black_rook & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'r';
            } else if (board.black_knight & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'n';
            } else if (board.black_bishop & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'b';
            } else if (board.black_queen & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'q';
            } else if (board.black_king & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'k';
            } else if (board.white_pawn & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'P';
            } else if (board.white_rook & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'R';
            } else if (board.white_knight & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'N';
            } else if (board.white_bishop & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'B';
            } else if (board.white_queen & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'Q';
            } else if (board.white_king & (1u64 << (i * 8 + j))) != 0 {
                piece_char = 'K';
            }

            board_string.push_str(&format!("{} ", piece_char));
        }
        board_string.push('\n');
    }
    board_string.push_str("    ----------------\n");
    board_string
}
