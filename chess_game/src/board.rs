
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

    pub fn _get_all_piece_mask(&self) -> u64 {
        self.get_black_pieces()|self.get_white_pieces()
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.white_pawn
            | self.white_rook
            | self.white_knight
            | self.white_bishop
            | self.white_queen
            | self.white_king
    }

    pub fn get_black_pieces(&self) -> u64 {
        self.black_pawn
            | self.black_rook
            | self.black_knight
            | self.black_bishop
            | self.black_queen
            | self.black_king
    }

    pub fn self_check_check(&self, from: u64, to: u64, is_white: bool) -> bool {
        return false;
    }

    pub fn illegal(&self, from: u64, to: u64, is_white: bool) -> bool {
        if is_white {
            if self.self_check_check(from, to, is_white) { // check if suicide
                return true;
            }
        } else {
            if self.self_check_check(from, to, is_white) { // check if suicide
                return true;
            }
        }
        return false;
    }

    pub fn move_piece(&mut self, from: u64, to: u64, is_white: bool) -> bool {
        // check who is moving
        if self.illegal(from, to, is_white) {
            return false;
        }

        if is_white {
            if (self.white_pawn | (1u64 << from)) == self.white_pawn {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    // Check if pawn can move there
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                        self.black_pawn = self.black_pawn & !(1u64 << to);
                    } else {
                        return false;
                    }
                } else {
                    // Check if pawn can move there
                    if (self.get_move_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                    } else {
                        return false;
                    }
                }
            }
        } else {            
            if (self.black_pawn | (1u64 << from)) == self.black_pawn {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    // Check if pawn can move there
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                        self.white_pawn = self.white_pawn & !(1u64 << to);
                    } else {
                        return false;
                    }
                } else {
                    if (self.get_move_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                    } else {
                        return false;
                    }
                }
            }

            // check if enemy occupies
            // modify enemy state
        }
        return true;
    }

    pub fn get_attack_mask(&self, pos: u64, is_white: bool) -> u64 {
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
                    return (1u64 << (pos-7)) & self.get_black_pieces();
                } else if (1u64 << pos | RIGHT_FILE_MASK) == RIGHT_FILE_MASK {
                    return (1u64 << (pos-9)) & self.get_black_pieces();
                } else {
                    return ((1u64 << (pos-9))|((1u64 << (pos-7)))) & self.get_black_pieces();
                }
            }
        } else {
            if ((self.black_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | LEFT_FILE_MASK) == LEFT_FILE_MASK  {
                    return (1u64 << (pos+9)) & self.get_white_pieces();
                } else if (1u64 << pos | RIGHT_FILE_MASK) == RIGHT_FILE_MASK {
                    return (1u64 << (pos+7)) & self.get_white_pieces();
                } else {
                    return ((1u64 << (pos+9))|((1u64 << (pos+7)))) & self.get_white_pieces();
                }
            }
        }
        
        return 0;
    }

    pub fn get_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        if is_white {
            if ((self.white_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | PAWN_WHITE_FIRST_MOVE_MASK) == PAWN_WHITE_FIRST_MOVE_MASK {
                    // pawn block jump
                    return ((1u64 << (pos-8))|((1u64 << (pos-16)))) & !self._get_all_piece_mask();
                } else {
                    return (1u64 << (pos-8)) & !self._get_all_piece_mask();
                }
            }
        } else {
            if ((self.black_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | PAWN_BLACK_FIRST_MOVE_MASK) == PAWN_BLACK_FIRST_MOVE_MASK {
                    // pawn block jump
                    return ((1u64 << (pos+8))|((1u64 << (pos+16)))) & !self._get_all_piece_mask();
                } else {
                    return (1u64 << (pos+8)) & !self._get_all_piece_mask();
                }            
            }
        }
        
        return 0;
    }

    pub fn display_board(&self) -> String {
        let rows = 8;
        let cols = 8;
        let mut board_string = String::new();
        board_string.push_str("    0 1 2 3 4 5 6 7\n");
        board_string.push_str("    ----------------\n");
    
        for i in 0..rows {
            board_string.push_str(&format!("{:2}| ", i*8));
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
        board_string.push_str("    ----------------\n");
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
    fn test_get_attack_mask_white_no_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_attack_mask(54, true);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_white_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_attack_mask(55, true);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_black_no_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_attack_mask(9, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_black_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_attack_mask(8, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_attack_mask(36, true);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_pawn_move_mask_white() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_move_mask(55, true);

        assert_eq!(result, 141287244169216);
    }

    #[test]
    fn test_get_pawn_move_mask_black() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_move_mask(15, false);

        assert_eq!(result, 2155872256);
    }

    #[test]
    fn test_get_pawn_move_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_move_mask(36, true);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_move_white_pawn_1_return_value() {
        let mut chessboard = Chessboard::new();
        let passed = chessboard.move_piece(55, 47, true);

        assert_eq!(passed, true);
    }

    #[test]
    fn test_move_white_pawn_1_board_value() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(55, 47, true);

        assert_eq!(chessboard.white_pawn, 35888059530608640);
    }

    #[test]
    fn test_move_black_pawn_2_board_value() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(9, 25, false);

        assert_eq!(chessboard.black_pawn, 33619200);
    }

    #[test]
    fn test_move_black_pawn_2_return_value_fail() {
        let mut chessboard = Chessboard::new();
        let passed = chessboard.move_piece(9, 26, false);

        assert_eq!(passed, false);
    }

    #[test]
    fn test_pawn_capture_collision() {
        let mut chessboard = Chessboard::new();
        
        chessboard.move_piece(51, 35, true);
        chessboard.move_piece(11, 27, false);
        // test if can move forwards if occupied
        chessboard.move_piece(51, 27, true);
        chessboard.move_piece(12, 28, false);
        // we capture with white
        chessboard.move_piece(35, 28, true);
        // we capture with black
        chessboard.move_piece(21, 28, false);
    
        
        assert_eq!(chessboard._get_all_piece_mask(), 18444210799321868287);
    }
}