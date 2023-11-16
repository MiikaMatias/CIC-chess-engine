
// Leftmost file mask: 72340172838076673
// Rightmost file mask: 9259542123273814144

const LEFT_FILE_MASK: u64 = 72340172838076673;
const RIGHT_FILE_MASK: u64 = 9259542123273814144;
const PAWN_WHITE_FIRST_MOVE_MASK: u64 = 71776119061217280;
const PAWN_BLACK_FIRST_MOVE_MASK: u64 = 65280;

pub struct Chessboard {
    pub white_pawn: u64, 
    pub white_rook: u64, 
    pub white_knight: u64, 
    pub white_bishop: u64, 
    pub white_queen: u64, 
    pub white_king: u64, 

    pub black_pawn: u64, 
    pub black_rook: u64, 
    pub black_knight: u64, 
    pub black_bishop: u64, 
    pub black_queen: u64, 
    pub black_king: u64, 

    pub white_turn: bool
}

impl Chessboard {
    pub fn new() -> Chessboard {
        Chessboard {
            black_pawn: 65280,   // Black pawns have a smaller number; they're at the "top"
            black_rook: 129, 
            black_knight: 66, 
            black_bishop: 36, 
            black_queen: 16, 
            black_king: 8, 
                                            // wwwww .... bbbbbb
            white_pawn: 71776119061217280, // White pawns have a larger number; "bottom"
            white_rook: 9295429630892703744, 
            white_knight: 4755801206503243776, 
            white_bishop: 2594073385365405696, 
            white_queen: 1152921504606846976, 
            white_king: 576460752303423488, 

            white_turn: true
        }
    }

    pub fn get_pawn_attack_mask(&self, pos: u64, is_white: bool) -> u64 {
        // Check if pawn on position
        /*   
POS 0 ->    r n b k q b n r
            p p p p p p p p                 When we >>, we actually move to the "left" on the board because we're moving towards the least significant bit
            e e e e e e e e 
            e e e e e e e e 
            e e e e e e e e 
            e e e e e e e e 
            P P P P P P P P
            R N B K Q B N R  <-  POS 63
         */
        if is_white {
            if ((self.white_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | LEFT_FILE_MASK) == LEFT_FILE_MASK {
                    return 1u64 << (pos-7);
                } else if (1u64 << pos | RIGHT_FILE_MASK) == RIGHT_FILE_MASK {
                    return 1u64 << (pos-9);
                } else {
                    return (1u64 << (pos-9))|((1u64 << (pos-7)));
                }
            }
        } else {
            if ((self.black_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | LEFT_FILE_MASK) == LEFT_FILE_MASK  {
                    return 1u64 << (pos+9);
                } else if (1u64 << pos | RIGHT_FILE_MASK) == RIGHT_FILE_MASK {
                    return 1u64 << (pos+7);
                } else {
                    return (1u64 << (pos+9))|((1u64 << (pos+7)));
                }
            }
        }
        
        return 0;
    }

    pub fn get_pawn_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        if is_white {
            if ((self.white_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | PAWN_WHITE_FIRST_MOVE_MASK) == PAWN_WHITE_FIRST_MOVE_MASK {
                    return (1u64 << (pos-8))|((1u64 << (pos-16)));
                } else {
                    return 1u64 << (pos-8);
                }
            }
        } else {
            if ((self.black_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | PAWN_BLACK_FIRST_MOVE_MASK) == PAWN_BLACK_FIRST_MOVE_MASK {
                    return (1u64 << (pos+8))|((1u64 << (pos+16)));
                } else {
                    return 1u64 << (pos+8);
                }            
            }
        }
        
        return 0;
    }

    pub fn display_board(&self) -> String {
        let rows = 8;
        let cols = 8;
        let mut board_string = String::new();
        
        for i in 0..rows {
            for j in 0..cols {
                let mut piece_char = 'e';
    
                if (self.black_pawn & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'p';
                } else if (self.black_rook & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'r';
                } else if (self.black_knight & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'n';
                } else if (self.black_bishop & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'b';
                } else if (self.black_queen & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'q';
                } else if (self.black_king & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'k';
                } else if (self.white_pawn & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'P';
                } else if (self.white_rook & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'R';
                } else if (self.white_knight & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'N';
                } else if (self.white_bishop & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'B';
                } else if (self.white_queen & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'Q';
                } else if (self.white_king & (1u64 << (i * 8 + j))) != 0 {
                    piece_char = 'K';
                }
    
                board_string.push_str(&format!("{} ", piece_char));
            }
            board_string.push_str("\n");
        }
        
        board_string
    }
    

}

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
        board_string.push_str("\n");
    }

    board_string
}


// TESTS


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pawn_attack_mask_white_no_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_attack_mask(54, true);

        assert_eq!(result, 175921860444160);
    }

    #[test]
    fn test_get_pawn_attack_mask_white_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_attack_mask(55, true);

        assert_eq!(result, 70368744177664);
    }

    #[test]
    fn test_get_pawn_attack_mask_black_no_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_attack_mask(9, false);

        assert_eq!(result, 327680);
    }

    #[test]
    fn test_get_pawn_attack_mask_black_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_attack_mask(8, false);

        assert_eq!(result, 131072);
    }

    #[test]
    fn test_get_pawn_attack_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_attack_mask(36, true);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_pawn_move_mask_white() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_move_mask(55, true);

        assert_eq!(result, 141287244169216);
    }

    #[test]
    fn test_get_pawn_move_mask_black() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_move_mask(15, false);

        assert_eq!(result, 2155872256);
    }

    #[test]
    fn test_get_pawn_move_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_pawn_move_mask(36, true);

        assert_eq!(result, 0);
    }

}
