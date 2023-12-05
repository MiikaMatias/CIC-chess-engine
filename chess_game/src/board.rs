
// Leftmost file mask: 72340172838076673
// Rightmost file mask: 9259542123273814144

const RANK_1_MASK: u64 = 18374686479671623680;
const RANK_2_MASK: u64 = 71776119061217280;
const RANK_7_MASK: u64 = 65280;
const RANK_8_MASK: u64 = 255;
const FILE_H_MASK: u64 = 9259542123273814144;
const FILE_G_MASK: u64 = 4629771061636907072;
const FILE_B_MASK: u64 = 144680345676153346;
const FILE_A_MASK: u64 = 72340172838076673;

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

    pub en_passant_square: u64,

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

            en_passant_square: 0,

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

    pub fn self_check_check(&self, _from: u64, _to: u64, _is_white: bool) -> bool {
        false
    }

    pub fn check_en_passant(&self, pos: u64, return_mask: u64, is_white: bool) -> u64 {
        if is_white {
            if self.en_passant_square == pos - 7 || self.en_passant_square == pos - 9 {
                return return_mask | self.en_passant_square;
            }
        } else if self.en_passant_square == pos + 7 || self.en_passant_square == pos + 9 {
            return return_mask | self.en_passant_square;
        }
        return_mask
    }

    pub fn illegal(&self, from: u64, to: u64, is_white: bool) -> bool {
        if is_white {
            if self.self_check_check(from, to, is_white) { // check if suicide
                return true;
            }
        } else if self.self_check_check(from, to, is_white) { // check if suicide
            return true;
        }
        false
    }

    pub fn take_piece_at_spot(&mut self, spot: u64, is_white: bool) {
        if is_white {
            if (self.black_pawn | (1u64 << spot)) == self.black_pawn {
                self.black_pawn &= !(1u64 << spot);
            } else if (self.black_knight | (1u64 << spot)) == self.black_knight {
                self.black_knight &= !(1u64 << spot);
            } else if (self.black_bishop | (1u64 << spot)) == self.black_bishop {
                self.black_bishop &= !(1u64 << spot);
            } else if (self.black_rook | (1u64 << spot)) == self.black_rook {
                self.black_rook &= !(1u64 << spot);
            } else if (self.black_queen | (1u64 << spot)) == self.black_queen {
                self.black_queen &= !(1u64 << spot);
            } else if (self.black_king | (1u64 << spot)) == self.black_king {
                self.black_king &= !(1u64 << spot);
            }
        } else if (self.white_pawn | (1u64 << spot)) == self.white_pawn {
            self.white_pawn &= !(1u64 << spot);
        } else if (self.white_knight | (1u64 << spot)) == self.white_knight {
            self.white_knight &= !(1u64 << spot);
        } else if (self.white_bishop | (1u64 << spot)) == self.white_bishop {
            self.white_bishop &= !(1u64 << spot);
        } else if (self.white_rook | (1u64 << spot)) == self.white_rook {
            self.white_rook &= !(1u64 << spot);
        } else if (self.white_queen | (1u64 << spot)) == self.white_queen {
            self.white_queen &= !(1u64 << spot);
        } else if (self.white_king | (1u64 << spot)) == self.white_king {
            self.white_king &= !(1u64 << spot);
        }
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
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    } 
                } else if (1u64 << to) == self.en_passant_square {
                    self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                    self.black_pawn &= !(1u64 << (to+8));
                    return true;
                } else {
                    // Check if pawn can move there
                    if (self.get_pawn_move_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                        if from - to == 16 {
                            // Set en_passant_square for the next move
                            self.en_passant_square = 1u64 << (to + 8);
                        } else {
                            // uncheck en passant if next move is pawn
                            self.en_passant_square = 0;
                        }
                        return true;
                   } 
                }
                self.en_passant_square = 0;
            } else if (self.white_knight | (1u64 << from)) == self.white_knight {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_knight = (self.white_knight & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_knight_move_mask(from) >> to) & 1u64 == 1 {
                        self.white_knight = (self.white_knight & !(1u64 << from)) | (1u64 << to);
                        return true;
                   } 
            } else if (self.white_rook | (1u64 << from)) == self.white_rook {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_rook = (self.white_rook & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_rook_move_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_rook = (self.white_rook & !(1u64 << from)) | (1u64 << to);
                        return true;
                    } 
            } else if (self.white_bishop | (1u64 << from)) == self.white_bishop {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_bishop = (self.white_bishop & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_bishop_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.white_bishop = (self.white_bishop & !(1u64 << from)) | (1u64 << to);
                    return true;
                    } 
            } else if (self.white_queen | (1u64 << from)) == self.white_queen {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_queen = (self.white_queen & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_queen_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.white_queen = (self.white_queen & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            }
        } else if (self.black_pawn | (1u64 << from)) == self.black_pawn {
            if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                // Check if pawn can move there
                if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                    self.white_pawn &= !(1u64 << to);
                    return true;
                }
            } else if (1u64 << to) == self.en_passant_square {
                self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                self.white_pawn &= !(1u64 << (to-8));
                return true;
            } else if (self.get_pawn_move_mask(from, is_white) >> to) & 1u64 == 1 {
                self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                if to - from == 16 {
                    // Set en_passant_square for the next move
                    self.en_passant_square = 1u64 << (to - 8);
                } else {
                    // uncheck en passant if next move is pawn
                    self.en_passant_square = 0;
                }
                return true;
            } 
        self.en_passant_square = 0;
        } else if (self.black_knight | (1u64 << from)) == self.black_knight {
            if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_knight = (self.black_knight & !(1u64 << from)) | (1u64 << to);
                    self.take_piece_at_spot(to, is_white);
                    return true;
                }  
            } else if (self.get_knight_move_mask(from) >> to) & 1u64 == 1 {
                    self.black_knight = (self.black_knight & !(1u64 << from)) | (1u64 << to);
                    return true;
               } 
            } else if (self.black_rook | (1u64 << from)) == self.black_rook {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_rook = (self.black_rook & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_rook_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_rook = (self.black_rook & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            } else if (self.black_bishop | (1u64 << from)) == self.black_bishop {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_bishop = (self.black_bishop & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_bishop_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_bishop = (self.black_bishop & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            } else if (self.black_queen | (1u64 << from)) == self.black_queen {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if (self.get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_queen = (self.black_queen & !(1u64 << from)) | (1u64 << to);
                        self.take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self.get_queen_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_queen = (self.black_queen & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            }
        false
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
                if (1u64 << pos | FILE_H_MASK) == FILE_H_MASK {
                    return self.check_en_passant(pos, (1u64 << (pos-9)) & self.get_black_pieces(), is_white);
                } else if (1u64 << pos | FILE_A_MASK) == FILE_A_MASK {
                    return  self.check_en_passant(pos, (1u64 << (pos-7)) & self.get_black_pieces(), is_white);
                } else {
                    return  self.check_en_passant(pos, ((1u64 << (pos-9))|(1u64 << (pos-7))) & self.get_black_pieces(), is_white);
                }
            } else if ((self.white_knight >> pos) & 1u64) == 1 {   
                return self.get_knight_move_mask(pos) & self.get_black_pieces()
            } else if ((self.white_rook >> pos) & 1u64) == 1 {   
                return self.get_rook_move_mask(pos, is_white) & self.get_black_pieces()
            } else if ((self.white_bishop >> pos) & 1u64) == 1 {   
                return self.get_bishop_move_mask(pos, is_white) & self.get_black_pieces()
            } else if ((self.white_queen >> pos) & 1u64) == 1 {   
                return self.get_queen_move_mask(pos, is_white) & self.get_black_pieces()
            }
        } else if ((self.black_pawn >> pos) & 1u64) == 1 {
            if (1u64 << pos | FILE_H_MASK) == FILE_H_MASK  {
                return  self.check_en_passant(pos, (1u64 << (pos+7)) & self.get_white_pieces(), is_white);
            } else if (1u64 << pos | FILE_A_MASK) == FILE_A_MASK {
                return  self.check_en_passant(pos, (1u64 << (pos+9)) & self.get_white_pieces(), is_white);
            } else {
                return  self.check_en_passant(pos, ((1u64 << (pos+9))|(1u64 << (pos+7))) & self.get_white_pieces(), is_white);
            }
            } else if ((self.black_knight >> pos) & 1u64) == 1 {   
                return self.get_knight_move_mask(pos) & self.get_white_pieces()
            } else if ((self.black_rook >> pos) & 1u64) == 1 {   
                return self.get_rook_move_mask(pos, is_white) & self.get_white_pieces()
            } else if ((self.black_bishop >> pos) & 1u64) == 1 {   
                return self.get_bishop_move_mask(pos, is_white) & self.get_white_pieces()
            } else if ((self.black_queen >> pos) & 1u64) == 1 {   
                return self.get_queen_move_mask(pos, is_white) & self.get_white_pieces()
            }
        
        0
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        let in_a_file = ((1u64 << pos) | FILE_A_MASK) == FILE_A_MASK;
        let in_b_file = ((1u64 << pos) | FILE_B_MASK) == FILE_B_MASK;
        let in_g_file = ((1u64 << pos) | FILE_G_MASK) == FILE_G_MASK;
        let in_h_file = ((1u64 << pos) | FILE_H_MASK) == FILE_H_MASK;
        let in_1_rank = ((1u64 << pos) | RANK_1_MASK) == RANK_1_MASK;
        let in_2_rank = ((1u64 << pos) | RANK_2_MASK) == RANK_2_MASK;
        let in_7_rank = ((1u64 << pos) | RANK_7_MASK) == RANK_7_MASK;
        let in_8_rank = ((1u64 << pos) | RANK_8_MASK) == RANK_8_MASK;

        if in_a_file {
            //done
            if in_8_rank {
                (1u64 << (pos+10)) | (1u64 << (pos+17))
            } else if in_7_rank {
                return (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6))
            } else if in_2_rank {
                return (1u64 << (pos-15)) | (1u64 << (pos+10)) | (1u64 << (pos-6))
            } else if in_1_rank {
                return (1u64 << (pos-15)) | (1u64 << (pos-6))
            } else {
                return (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6)) | (1u64 << (pos-15))
            }
        } else if in_b_file {
            if in_8_rank {
                return (1u64 << (pos+15)) |  (1u64 << (pos+10)) |  (1u64 << (pos+17))
            } else if in_7_rank {
                return (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6)) | (1u64 << (pos-6))
            } else if in_2_rank {
                return (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6)) | (1u64 << (pos+10))
            } else if in_1_rank {
                return (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6))
            } else {
                return (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
            }
        } else if in_g_file {
            if in_8_rank {
                return (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6))
            } else if in_7_rank {
                return (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) 
            } else if in_2_rank {
                return (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos+6)) | (1u64 << (pos-10))
            } else if in_1_rank {
                return (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-10))
            } else {
                return (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
            }
        } else if in_h_file {
            if in_8_rank {
                return (1u64 << (pos+15)) | (1u64 << (pos+6))
            } else if in_7_rank {
                return (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) 
            } else if in_2_rank {
                return (1u64 << (pos-17)) | (1u64 << (pos+6)) | (1u64 << (pos-10))
            } else if in_1_rank {
                return (1u64 << (pos-10)) | (1u64 << (pos-17))
            } else {
                return (1u64 << (pos+6)) | (1u64 << (pos+15)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
            }
        } else {
            //done
            if in_8_rank {
                return (1u64 << (pos+10)) | (1u64 << (pos+17)) | (1u64 << (pos+6)) | (1u64 << (pos+15))
            } else if in_7_rank {
                return (1u64 << (pos+10)) | (1u64 << (pos+17)) | (1u64 << (pos+6)) | (1u64 << (pos+15)) | (1u64 << (pos-6))  | (1u64 << (pos-10))
            } else if in_2_rank {
                return (1u64 << (pos-10)) | (1u64 << (pos-17)) | (1u64 << (pos-6)) | (1u64 << (pos-15))  | (1u64 << (pos+6))  | (1u64 << (pos+10))
            } else if in_1_rank {
                return (1u64 << (pos-10)) | (1u64 << (pos-17)) | (1u64 << (pos-6)) | (1u64 << (pos-15))
            } else {
                return (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos+10)) | (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6)) | (1u64 << (pos-10)) 
            }
        }
    }

    pub fn get_rook_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        let mut board: u64 = 0;
    
        if is_white {
            // Generate horizontal moves to the left
            for i in (0..(pos % 8)).rev() {
                let square = pos / 8 * 8 + i;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
        
            // Generate horizontal moves to the right
            for i in ((pos % 8) + 1)..8 {
                let square = pos / 8 * 8 + i;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
        
            // Generate vertical moves upwards
            for i in (0..(pos / 8)).rev() {
                let square = i * 8 + pos % 8;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
        
            // Generate vertical moves downwards
            for i in ((pos / 8) + 1)..8 {
                let square = i * 8 + pos % 8;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;       
                    break;
                }
            }

            board & !self.get_white_pieces()
        } else {
            // Generate horizontal moves to the left
            for i in (0..(pos % 8)).rev() {
                let square = pos / 8 * 8 + i;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
        
            // Generate horizontal moves to the right
            for i in ((pos % 8) + 1)..8 {
                let square = pos / 8 * 8 + i;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
        
            // Generate vertical moves upwards
            for i in (0..(pos / 8)).rev() {
                let square = i * 8 + pos % 8;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
        
            // Generate vertical moves downwards
            for i in ((pos / 8) + 1)..8 {
                let square = i * 8 + pos % 8;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;       
                    break;
                }
            }
            board & !self.get_black_pieces()
        }    
        
    }
    
    pub fn get_bishop_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        let mut board: u64 = 0;
    
        if is_white {
            // Generate moves to the top-left
            for i in 1..8 {
                let file = (pos % 8) as i64 - i;
                let rank = (pos / 8) as i64 - i;
                if file < 0 || rank < 0 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            // Generate moves to the top-right
            for i in 1..8 {
                let file = (pos % 8) as i64 + i;
                let rank = (pos / 8) as i64 - i;
                if file >= 8 || rank < 0 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            // Generate moves to the bottom-left
            for i in 1..8 {
                let file = (pos % 8) as i64 - i;
                let rank = (pos / 8) as i64 + i;
                if file < 0 || rank >= 8 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            // Generate moves to the bottom-right
            for i in 1..8 {
                let file = (pos % 8) as i64 + i;
                let rank = (pos / 8) as i64 + i;
                if file >= 8 || rank >= 8 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            board & !self.get_white_pieces()
        } else {
            // Generate moves to the top-left
            for i in 1..8 {
                let file = (pos % 8) as i64 - i;
                let rank = (pos / 8) as i64 - i;
                if file < 0 || rank < 0 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            // Generate moves to the top-right
            for i in 1..8 {
                let file = (pos % 8) as i64 + i;
                let rank = (pos / 8) as i64 - i;
                if file >= 8 || rank < 0 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            // Generate moves to the bottom-left
            for i in 1..8 {
                let file = (pos % 8) as i64 - i;
                let rank = (pos / 8) as i64 + i;
                if file < 0 || rank >= 8 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            // Generate moves to the bottom-right
            for i in 1..8 {
                let file = (pos % 8) as i64 + i;
                let rank = (pos / 8) as i64 + i;
                if file >= 8 || rank >= 8 {
                    break;
                }
                let square = (rank * 8 + file) as u64;
                if (self._get_all_piece_mask() & (1 << square)) == 0 {
                    board |= 1 << square;
                } else {
                    board |= 1 << square;
                    break;
                }
            }
    
            board & !self.get_black_pieces()
        }
    }    

    pub fn get_queen_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        self.get_rook_move_mask(pos, is_white) | self.get_bishop_move_mask(pos, is_white)
    }

    pub fn get_pawn_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        if is_white {
            if ((self.white_pawn >> pos) & 1u64) == 1 {
                if (1u64 << pos | RANK_2_MASK) == RANK_2_MASK {
                    // check for piece in the way
                    if 1u64 << (pos-8) | self._get_all_piece_mask() == self._get_all_piece_mask(){
                        return 0;
                    }
                    return ((1u64 << (pos-8))|(1u64 << (pos-16))) & !self._get_all_piece_mask();
                } else {
                    return (1u64 << (pos-8)) & !self._get_all_piece_mask();
                }
            }
        } else if ((self.black_pawn >> pos) & 1u64) == 1 {
            if (1u64 << pos | RANK_7_MASK) == RANK_7_MASK {
                if 1u64 << (pos+8) | self._get_all_piece_mask() == self._get_all_piece_mask(){
                    return 0;
                }                 
                return ((1u64 << (pos+8))|(1u64 << (pos+16))) & !self._get_all_piece_mask();
            } else {
                return (1u64 << (pos+8)) & !self._get_all_piece_mask();
            }     
        }      
        0
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
            board_string.push('\n');
        }
        board_string.push_str("    ----------------\n");
        board_string
    }
    

}

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

    #[test]
    fn test_move_white_pawn_1_return_value() {
        let mut chessboard = Chessboard::new();
        let passed = chessboard.move_piece(55, 47, true);

        assert!(passed);
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

        assert!(!passed);
    }

    #[test]
    fn test_white_pawn_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(48, 32, true);
        chessboard.move_piece(32, 24, true);
        chessboard.move_piece(24, 16, true);
        chessboard.move_piece(16, 9, true);
        assert_eq!(chessboard._get_all_piece_mask(), 18446181123756195839);
    }

    #[test]
    fn test_black_pawn_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(9,25, false);
        chessboard.move_piece(25, 33, false);
        chessboard.move_piece(33, 41, false);
        chessboard.move_piece(41, 48, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598732905983);
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

    #[test]
    fn test_pawn_jump_not_allowed() {
        let mut chessboard = Chessboard::new();
        
        // bring white pawn to front of black pieces
        chessboard.move_piece(51, 35, true);
        chessboard.move_piece(35, 27, true);
        chessboard.move_piece(27, 19, true);
        // bring black pawn to front of white pieces
        chessboard.move_piece(8, 24, false);
        chessboard.move_piece(24, 32, false);
        chessboard.move_piece(32, 40, false);

        //JUMP!
        chessboard.move_piece(48, 32, true);
        chessboard.move_piece(11, 27, false);

        assert_eq!(chessboard._get_all_piece_mask(), 18444211898431373055);
    }

    #[test]
    fn test_en_passant_square() {
        let mut chessboard = Chessboard::new();
        // create square
        chessboard.move_piece(48, 32, true);
        assert_eq!(chessboard.en_passant_square, (1u64 << 40));
        // make sure it's gone
        chessboard.move_piece(32, 24, true);
        assert_eq!(chessboard.en_passant_square, 0);
    }

    #[test]
    fn test_en_passant_allowed() {
        let mut chessboard = Chessboard::new();
        // bring white pawn to front of black pieces
        chessboard.move_piece(51, 35, true);
        chessboard.move_piece(35, 27, true);
        
        // bring black pawn to front of white pieces
        chessboard.move_piece(8, 24, false);
        chessboard.move_piece(24, 32, false);

        //JUMP!
        chessboard.move_piece(49, 33, true);
        // eat
        let epb: bool =chessboard.move_piece(32, 41, false);
        //JUMP!
        chessboard.move_piece(12, 28, false);
        //eat
        let epw =chessboard.move_piece(27, 20, true);
        assert_eq!(chessboard._get_all_piece_mask(), 18443650047990099711);
        assert!(epb);
        assert!(epw);
    }

    #[test]
    fn test_white_knight_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(57, 40, true);
        chessboard.move_piece(40, 25, true);
        chessboard.move_piece(25, 8, true);
        assert_eq!(chessboard._get_all_piece_mask(), 18302347410657050623)
    }

    #[test]
    fn test_black_knight_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(1,18, false);
        chessboard.move_piece(18, 35, false);
        chessboard.move_piece(35, 52, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598732906493)
    }

    #[test]
    fn test_get_knight_move_masks() {
        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(0);
        assert_eq!(result, 132096);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(1);
        assert_eq!(result, 329728);
        
        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(2);
        assert_eq!(result, 659712);
    
        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(6);
        assert_eq!(result, 10489856);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(7);
        assert_eq!(result, 4202496);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(14);
        assert_eq!(result, 2685403152);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(15);
        assert_eq!(result, 1075839008);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(24);
        assert_eq!(result, 2216203387392);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(31);
        assert_eq!(result, 70506185244672);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(32);
        assert_eq!(result, 567348067172352);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(39);
        assert_eq!(result, 18049583422636032);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(48);
        assert_eq!(result, 288234782788157440);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(49);
        assert_eq!(result, 576469569871282176);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(54);
        assert_eq!(result, 1152939783987658752);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(55);
        assert_eq!(result, 2305878468463689728);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(56);
        assert_eq!(result, 1128098930098176);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(57);
        assert_eq!(result, 2257297371824128);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(58);
        assert_eq!(result, 4796069720358912);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(62);
        assert_eq!(result, 4679521487814656);

        let chessboard = Chessboard::new();
        let result = chessboard.get_knight_move_mask(63);
        assert_eq!(result, 9077567998918656);
    }

    #[test]
    fn test_rook_move_mask() {
        let mut chessboard = Chessboard::new();
        chessboard.get_pawn_move_mask(55, true);
        chessboard.get_pawn_move_mask(15, false);
        chessboard.move_piece(51, 35, true);
        chessboard.move_piece(35, 27, true);
        chessboard.move_piece(8, 24, false);
        chessboard.move_piece(24, 32, false);
        chessboard.move_piece(49, 33, true);
        chessboard.move_piece(32, 41, false);
        chessboard.move_piece(12, 28, false);
        chessboard.move_piece(27, 20, true);
        let result = chessboard.get_rook_move_mask(47, true);
        assert_eq!(result, 139090376818688);
        let result = chessboard.get_rook_move_mask(47, false);
        assert_eq!(result, 36165688372494336);
    }

    #[test]
    fn test_white_rook_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(51, 35, true);
        chessboard.move_piece(35, 27, true);
        chessboard.move_piece(8, 24, false);
        chessboard.move_piece(24, 32, false);
        chessboard.move_piece(49, 33, true);
        chessboard.move_piece(32, 41, false);
        chessboard.move_piece(12, 28, false);
        chessboard.move_piece(27, 20, true);
        chessboard.move_piece(55, 39, true);
        chessboard.move_piece(63, 47, true);
        chessboard.move_piece(47, 41, true);
    
        assert_eq!(chessboard._get_all_piece_mask(), 9184249763872173823);
    }

    #[test]
    fn test_black_rook_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(8, 24, false);
        chessboard.move_piece(0, 16, false);
        chessboard.move_piece(16, 23, false);
        chessboard.move_piece(23, 55, false);
    
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598749683454);
    }

    #[test]
    fn test_white_bishop_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(49, 41, true);
        chessboard.move_piece(58, 40, true);
        chessboard.move_piece(40, 12, true);
        assert_eq!(chessboard._get_all_piece_mask(), 18157671471651028991);
    }

    #[test]
    fn test_black_bishop_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(49, 41, true);
        chessboard.move_piece(58, 40, true);
        chessboard.move_piece(40, 12, true);
        chessboard.move_piece(5, 12, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18157671471651028959);
    }

    #[test]
    fn test_queen_movement_capture() {
        let mut chessboard = Chessboard::new();
        chessboard.move_piece(51, 35, true);
        chessboard.move_piece(60, 24, true);
        chessboard.move_piece(24, 10, true);
        chessboard.move_piece(10, 1, true);
        chessboard.move_piece(1, 2, true);
        chessboard.move_piece(2, 3, true);
        chessboard.move_piece(4, 3, false);
        assert_eq!(chessboard._get_all_piece_mask(), 17291289328672111593)
    }


}