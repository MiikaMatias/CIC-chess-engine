pub struct Chessboard {
    white_pawn: u64, 
    white_rook: u64, 
    white_knight: u64, 
    white_bishop: u64, 
    white_queen: u64, 
    white_king: u64, 

    black_pawn: u64, 
    black_rook: u64, 
    black_knight: u64, 
    black_bishop: u64, 
    black_queen: u64, 
    black_king: u64, 
}

impl Chessboard {
    pub fn new() -> Chessboard {
        Chessboard {
            white_pawn: 65280, 
            white_rook: 129, 
            white_knight: 66, 
            white_bishop: 36, 
            white_queen: 16, 
            white_king: 8, 
        
            black_pawn: 71776119061217280, 
            black_rook: 9295429630892703744, 
            black_knight: 4755801206503243776, 
            black_bishop: 2594073385365405696, 
            black_queen: 1152921504606846976, 
            black_king: 576460752303423488, 
        }
    }

    pub fn get_positions(&self) -> Vec<(u64, char)> {
        vec![
            (self.white_pawn, 'P'),
            (self.white_rook, 'R'),
            (self.white_knight, 'N'),
            (self.white_bishop, 'B'),
            (self.white_queen, 'Q'),
            (self.white_king, 'K'),
            (self.black_pawn, 'p'),
            (self.black_rook, 'r'),
            (self.black_knight, 'n'),
            (self.black_bishop, 'b'),
            (self.black_queen, 'q'),
            (self.black_king, 'k'),
        ]
    }
    
    
    pub fn display_board(&self) -> String {
        let rows = 8;
        let cols = 8;

        let mut board_string = String::new();
        for i in 0..rows {
            for j in 0..cols {
                let mut piece_char = 'e'; // Default empty square
                for &(piece, piece_type) in self.get_positions().iter() {
                    if (piece & (1u64 << (i * 8 + j))) != 0 {
                        piece_char = piece_type;
                        break;
                    }
                }
                board_string.push_str(&format!("{} ", piece_char));
            }
            board_string.push_str("\n");
        }
    
        return board_string;
    }
}