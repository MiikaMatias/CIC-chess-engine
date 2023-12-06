
// Leftmost file mask: 72340172838076673
// Rightmost file mask: 9259542123273814144
#[allow(dead_code)]
const RANK_1_MASK: u64 = 18374686479671623680;
#[allow(dead_code)]
const RANK_2_MASK: u64 = 71776119061217280;
#[allow(dead_code)]
const RANK_7_MASK: u64 = 65280;
#[allow(dead_code)]
const RANK_8_MASK: u64 = 255;
#[allow(dead_code)]
const FILE_H_MASK: u64 = 9259542123273814144;
#[allow(dead_code)]
const FILE_G_MASK: u64 = 4629771061636907072;
#[allow(dead_code)]
const FILE_B_MASK: u64 = 144680345676153346;
#[allow(dead_code)]
const FILE_A_MASK: u64 = 72340172838076673;

#[derive(Copy, Clone)]
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

    pub white_turn: bool,

    pub white_castle_allowed_left: bool,
    pub white_castle_allowed_right: bool,
    pub black_castle_allowed_left: bool,
    pub black_castle_allowed_right: bool,
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

            white_turn: true,

            white_castle_allowed_left: true,
            white_castle_allowed_right: true,
            black_castle_allowed_left: true,
            black_castle_allowed_right: true
        }
    }

    pub fn _get_all_moves_at_position(&self, pos: u64, is_white: bool) -> Vec<u64> {
        if is_white {
            if (self.white_pawn & (1u64 << pos)) == (1u64 << pos) {
                find_set_bits_positions(self._get_pawn_move_mask(pos, is_white)& !self.get_white_pieces())
            } else if (self.white_rook & (1u64 << pos)) == (1u64 << pos) {
                return find_set_bits_positions(self._get_rook_move_mask(pos, is_white)& !self.get_white_pieces());
            } else if (self.white_bishop & (1u64 << pos)) == (1u64 << pos) {
                return find_set_bits_positions(self._get_bishop_move_mask(pos, is_white)& !self.get_white_pieces());
            } else if (self.white_king & (1u64 << pos)) == (1u64 << pos) {
                return find_set_bits_positions(self._get_king_move_mask(pos)& !self.get_white_pieces());
            } else if (self.white_knight & (1u64 << pos)) == (1u64 << pos) {
                return find_set_bits_positions(self._get_knight_move_mask(pos)& !self.get_white_pieces());
            } else {
                return find_set_bits_positions(self._get_queen_move_mask(pos, is_white)& !self.get_white_pieces());
            }
        } else if (self.black_pawn & (1u64 << pos)) == (1u64 << pos) {
            find_set_bits_positions(self._get_pawn_move_mask(pos, is_white)& !self.get_black_pieces())
        } else if (self.black_rook & (1u64 << pos)) == (1u64 << pos) {
            return find_set_bits_positions(self._get_rook_move_mask(pos, is_white)& !self.get_black_pieces());
        } else if (self.black_bishop & (1u64 << pos)) == (1u64 << pos) {
            return find_set_bits_positions(self._get_bishop_move_mask(pos, is_white)& !self.get_black_pieces());
        } else if (self.black_king & (1u64 << pos)) == (1u64 << pos) {
            return find_set_bits_positions(self._get_king_move_mask(pos)& !self.get_black_pieces());
        } else if (self.black_knight & (1u64 << pos)) == (1u64 << pos) {
            return find_set_bits_positions(self._get_knight_move_mask(pos)& !self.get_black_pieces());
        } else{
            return find_set_bits_positions(self._get_queen_move_mask(pos, is_white)& !self.get_black_pieces());
        }
    }

    pub fn _get_all_possible_moves(&self, is_white: bool) -> Vec<Chessboard> {
        // this will very likely get rough with memory; consider having an array of values instead

        let mut board_array: Vec<Chessboard> = Vec::new();
        let _i = 0;

        let pieces = if is_white {
            [self.white_pawn, self.white_bishop, self.white_king, self.white_queen, self.white_rook, self.white_knight]
        } else {
            [self.black_pawn, self.black_bishop, self.black_king, self.black_queen, self.black_rook, self.black_knight]
        };

        for piece in pieces {
            let positions_of_pieces = find_set_bits_positions(piece);
            for position in positions_of_pieces {
                let moves_of_position = self._get_all_moves_at_position(position, is_white);
                for move_target in moves_of_position {
                    let mut new_chessboard = *self;
                    let is_legal = new_chessboard._move_piece(position, move_target, is_white, true);
                    if is_legal {
                        board_array.push(new_chessboard);
                    }
                }
            }
        }

        board_array
    }

    pub fn _get_all_piece_mask(&self) -> u64 {
        self.get_black_pieces()|self.get_white_pieces()
    }

    pub fn _get_threat_masks(&self, is_white: bool) -> [u64; 6] {
        
        let mut threat_mask: [u64; 6] = [0; 6]; 

        if is_white {
            for j in 0..8 {
                for i in 1..9 {
                    let check_square = (j*8) + i - 1;
                    if (self.white_pawn | (1u64 << check_square))  == self.white_pawn {
                        threat_mask[0] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    } 
                    if (self.white_bishop | (1u64 << check_square))  == self.white_bishop {
                        threat_mask[1] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.white_knight | (1u64 << check_square))  == self.white_knight {
                        threat_mask[2] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.white_rook | (1u64 << check_square))  == self.white_rook {
                        threat_mask[3] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.white_queen | (1u64 << check_square))  == self.white_queen {
                        threat_mask[4] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.white_king | (1u64 << check_square))  == self.white_king {
                        threat_mask[5] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    } 
                }
            }
        } else {
            for j in 0..8 {
                for i in 1..9 {
                    let check_square = (j*8) + i - 1;
                    if (self.black_pawn | (1u64 << check_square))  == self.black_pawn {
                        threat_mask[0] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    } 
                    if (self.black_bishop | (1u64 << check_square))  == self.black_bishop {
                        threat_mask[1] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.black_knight | (1u64 << check_square))  == self.black_knight {
                        threat_mask[2] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.black_rook | (1u64 << check_square))  == self.black_rook {
                        threat_mask[3] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.black_queen | (1u64 << check_square))  == self.black_queen {
                        threat_mask[4] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    }
                    if (self.black_king | (1u64 << check_square))  == self.black_king {
                        threat_mask[5] |= self._get_attack_mask(check_square, is_white);
                        continue;
                    } 
                }
            }
        }
        threat_mask
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

    pub fn _self_check_check(&self, from: u64, to: u64, is_white: bool) -> bool {
        let mut next_state = *self;
        next_state._move_piece(from, to, is_white, false);

        let opponent_threatens: u64 = next_state._get_threat_masks(!is_white).iter().cloned().fold(0, |acc, x| acc | x);

        if opponent_threatens == 0 {
            return false;
        }

        if is_white {
            (self.white_king & opponent_threatens) == self.white_king
        } else {
            (self.black_king & opponent_threatens) == self.black_king
        }
    }

    pub fn _check_en_passant(&self, pos: u64, return_mask: u64, is_white: bool) -> u64 {
        if is_white {
            if self.en_passant_square == pos - 7 || self.en_passant_square == pos - 9 {
                return return_mask | self.en_passant_square;
            }
        } else if self.en_passant_square == pos + 7 || self.en_passant_square == pos + 9 {
            return return_mask | self.en_passant_square;
        }
        return_mask
    }

    pub fn _take_piece_at_spot(&mut self, spot: u64, is_white: bool) {
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
        }
    }

    
    pub fn _move_piece(&mut self, from: u64, to: u64, is_white: bool, care_about_illegality:bool) -> bool {

        // check who is moving
        if care_about_illegality && self._self_check_check(from, to, is_white) {
            return false;
        }  

        if is_white {
            if (self.white_pawn | (1u64 << from)) == self.white_pawn {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        if ((1u64 << to) | RANK_8_MASK) == RANK_8_MASK {
                            self.white_queen |= 1u64 << to;
                            self.white_pawn &= !(1u64 << from);
                        } else {
                            self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                        }
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    } 
                } else if (1u64 << to) == self.en_passant_square {
                    self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                    self.black_pawn &= !(1u64 << (to+8));
                    return true;
                }
                if (self._get_pawn_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    if ((1u64 << to) | RANK_8_MASK) == RANK_8_MASK {
                        self.white_queen |= 1u64 << to;
                        self.white_pawn &= !(1u64 << from);
                    } else {
                        self.white_pawn = (self.white_pawn & !(1u64 << from)) | (1u64 << to);
                    }
                    if from - to == 16 {
                        // Set en_passant_square for the next move
                        self.en_passant_square = 1u64 << (to + 8);
                    } else {
                        // uncheck en passant if next move is pawn
                        self.en_passant_square = 0;
                    }
                    return true;
                } 
                self.en_passant_square = 0;
            } else if (self.white_knight | (1u64 << from)) == self.white_knight {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_knight = (self.white_knight & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_knight_move_mask(from) >> to) & 1u64 == 1 {
                        self.white_knight = (self.white_knight & !(1u64 << from)) | (1u64 << to);
                        return true;
                   } 
            } else if (self.white_rook | (1u64 << from)) == self.white_rook {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_rook = (self.white_rook & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_rook_move_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_rook = (self.white_rook & !(1u64 << from)) | (1u64 << to);
                        return true;
                    } 
            } else if (self.white_bishop | (1u64 << from)) == self.white_bishop {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_bishop = (self.white_bishop & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_bishop_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.white_bishop = (self.white_bishop & !(1u64 << from)) | (1u64 << to);
                    return true;
                    } 
            } else if (self.white_queen | (1u64 << from)) == self.white_queen {
                if self.get_black_pieces() | (1u64 << to) == self.get_black_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_queen = (self.white_queen & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_queen_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.white_queen = (self.white_queen & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            } else if (self.white_king | (1u64 << from)) == self.white_king {
                let old_king = self.white_king;
                self.white_king = (self.white_king & !(1u64 << from)) | (1u64 << to);
                let threats = self._get_threat_masks(!is_white).iter().cloned().fold(0, |acc, x| acc | x);
                if (threats & self.white_king) == self.white_king {
                    self.white_king = old_king;
                    return false;
                }
                self.white_king = old_king;
                
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if  (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.white_king = (self.white_king & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if ((self._get_king_move_mask(from)) >> to) & 1u64 == 1 {
                    self.white_king = (self.white_king & !(1u64 << from)) | (1u64 << to) ;
                    return true;
                }
            }
        } else if (self.black_pawn | (1u64 << from)) == self.black_pawn {
            if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                if ((1u64 << to) | RANK_1_MASK) == RANK_1_MASK {
                    self.black_queen |= 1u64 << to;
                    self.black_pawn &= !(1u64 << from);
                } else {
                    self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                }

                if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                    self.white_pawn &= !(1u64 << to);
                    return true;
                }
            } else if (1u64 << to) == self.en_passant_square {
                self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                self.white_pawn &= !(1u64 << (to-8));
                return true;
            } else if (self._get_pawn_move_mask(from, is_white) >> to) & 1u64 == 1 {
                if ((1u64 << to) | RANK_1_MASK) == RANK_1_MASK {
                    self.black_queen |= 1u64 << to;
                    self.black_pawn &= !(1u64 << from);
                } else {
                    self.black_pawn = (self.black_pawn & !(1u64 << from)) | (1u64 << to);
                }

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
                if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_knight = (self.black_knight & !(1u64 << from)) | (1u64 << to);
                    self._take_piece_at_spot(to, is_white);
                    return true;
                }  
            } else if (self._get_knight_move_mask(from) >> to) & 1u64 == 1 {
                    self.black_knight = (self.black_knight & !(1u64 << from)) | (1u64 << to);
                    return true;
               } 
            } else if (self.black_rook | (1u64 << from)) == self.black_rook {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_rook = (self.black_rook & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_rook_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_rook = (self.black_rook & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            } else if (self.black_bishop | (1u64 << from)) == self.black_bishop {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_bishop = (self.black_bishop & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_bishop_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_bishop = (self.black_bishop & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            } else if (self.black_queen | (1u64 << from)) == self.black_queen {
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_queen = (self.black_queen & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if (self._get_queen_move_mask(from, is_white) >> to) & 1u64 == 1 {
                    self.black_queen = (self.black_queen & !(1u64 << from)) | (1u64 << to);
                    return true;
                }
            } else if (self.black_king | (1u64 << from)) == self.black_king {
                let old_king = self.black_king;
                self.black_king = (self.black_king & !(1u64 << from)) | (1u64 << to);
                let threats = self._get_threat_masks(!is_white).iter().cloned().fold(0, |acc, x| acc | x);
                if (threats & self.black_king) == self.black_king {
                    self.black_king = old_king;
                    return false;
                }
                self.black_king = old_king;
                
                if self.get_white_pieces() | (1u64 << to) == self.get_white_pieces() {
                    if  (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                        self.black_king = (self.black_king & !(1u64 << from)) | (1u64 << to);
                        self._take_piece_at_spot(to, is_white);
                        return true;
                    }  
                } else if ((self._get_king_move_mask(from)) >> to) & 1u64 == 1 {
                    self.black_king = (self.black_king & !(1u64 << from)) | (1u64 << to) ;
                    return true;
                }
            }
        false
    }
        
    pub fn _get_attack_mask(&self, pos: u64, is_white: bool) -> u64 {
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
                    return self._check_en_passant(pos, (1u64 << (pos-9)) & self.get_black_pieces(), is_white);
                } else if (1u64 << pos | FILE_A_MASK) == FILE_A_MASK {
                    return  self._check_en_passant(pos, (1u64 << (pos-7)) & self.get_black_pieces(), is_white);
                } else {
                    return  self._check_en_passant(pos, ((1u64 << (pos-9))|(1u64 << (pos-7))) & self.get_black_pieces(), is_white);
                }
            } else if ((self.white_knight >> pos) & 1u64) == 1 {   
                return self._get_knight_move_mask(pos) & self.get_black_pieces()
            } else if ((self.white_rook >> pos) & 1u64) == 1 {   
                return self._get_rook_move_mask(pos, is_white) & self.get_black_pieces()
            } else if ((self.white_bishop >> pos) & 1u64) == 1 {   
                return self._get_bishop_move_mask(pos, is_white) & self.get_black_pieces()
            } else if ((self.white_queen >> pos) & 1u64) == 1 {   
                return self._get_queen_move_mask(pos, is_white) & self.get_black_pieces()
            } else if ((self.white_king >> pos) & 1u64) == 1 {   
                return self._get_king_move_mask(pos) & self.get_black_pieces()
            } 
            return 0;
        }
        if ((self.black_pawn >> pos) & 1u64) == 1 {
            if (1u64 << pos | FILE_H_MASK) == FILE_H_MASK  {
                return  self._check_en_passant(pos, (1u64 << (pos+7)) & self.get_white_pieces(), is_white);
            } else if (1u64 << pos | FILE_A_MASK) == FILE_A_MASK {
                return  self._check_en_passant(pos, (1u64 << (pos+9)) & self.get_white_pieces(), is_white);
            } else {
                return  self._check_en_passant(pos, ((1u64 << (pos+9))|(1u64 << (pos+7))) & self.get_white_pieces(), is_white);
            }
        } else if ((self.black_knight >> pos) & 1u64) == 1 {   
            return self._get_knight_move_mask(pos) & self.get_white_pieces()
        } else if ((self.black_rook >> pos) & 1u64) == 1 {   
            return self._get_rook_move_mask(pos, is_white) & self.get_white_pieces()
        } else if ((self.black_bishop >> pos) & 1u64) == 1 {   
            return self._get_bishop_move_mask(pos, is_white) & self.get_white_pieces()
        } else if ((self.black_queen >> pos) & 1u64) == 1 {   
            return self._get_queen_move_mask(pos, is_white) & self.get_white_pieces()
        } else if ((self.black_king >> pos) & 1u64) == 1 {   
            return self._get_king_move_mask(pos) & self.get_white_pieces()
        }
        0
    }

    pub fn _get_knight_move_mask(&self, pos: u64) -> u64 {
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
                (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6))
            } else if in_2_rank {
                (1u64 << (pos-15)) | (1u64 << (pos+10)) | (1u64 << (pos-6))
            } else if in_1_rank {
                (1u64 << (pos-15)) | (1u64 << (pos-6))
            } else {
                (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6)) | (1u64 << (pos-15))
            }
        } else if in_b_file {
            if in_8_rank {
                (1u64 << (pos+15)) |  (1u64 << (pos+10)) |  (1u64 << (pos+17))
            } else if in_7_rank {
                (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6)) | (1u64 << (pos-6))
            } else if in_2_rank {
                (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6)) | (1u64 << (pos+10))
            } else if in_1_rank {
                (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6))
            } else {
                (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
            }
        } else if in_g_file {
            if in_8_rank {
                (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6))
            } else if in_7_rank {
                (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) 
            } else if in_2_rank {
                (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos+6)) | (1u64 << (pos-10))
            } else if in_1_rank {
                (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-10))
            } else {
                (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
            }
        } else if in_h_file {
            if in_8_rank {
                (1u64 << (pos+15)) | (1u64 << (pos+6))
            } else if in_7_rank {
                (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) 
            } else if in_2_rank {
                (1u64 << (pos-17)) | (1u64 << (pos+6)) | (1u64 << (pos-10))
            } else if in_1_rank {
                (1u64 << (pos-10)) | (1u64 << (pos-17))
            } else {
                (1u64 << (pos+6)) | (1u64 << (pos+15)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
            }
        } else {
            //done
            if in_8_rank {
                (1u64 << (pos+10)) | (1u64 << (pos+17)) | (1u64 << (pos+6)) | (1u64 << (pos+15))
            } else if in_7_rank {
                (1u64 << (pos+10)) | (1u64 << (pos+17)) | (1u64 << (pos+6)) | (1u64 << (pos+15)) | (1u64 << (pos-6))  | (1u64 << (pos-10))
            } else if in_2_rank {
                (1u64 << (pos-10)) | (1u64 << (pos-17)) | (1u64 << (pos-6)) | (1u64 << (pos-15))  | (1u64 << (pos+6))  | (1u64 << (pos+10))
            } else if in_1_rank {
                (1u64 << (pos-10)) | (1u64 << (pos-17)) | (1u64 << (pos-6)) | (1u64 << (pos-15))
            } else {
                (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos+10)) | (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6)) | (1u64 << (pos-10)) 
            }
        }
    }

    pub fn _get_rook_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        let mut board: u64 = 0;

        // Replace with precomputed values to improve performance
    
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
    
    pub fn _get_bishop_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        let mut board: u64 = 0;
    
        // Replace with precomputed values to improve performance

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

    pub fn _get_queen_move_mask(&self, pos: u64, is_white: bool) -> u64 {
        self._get_rook_move_mask(pos, is_white) | self._get_bishop_move_mask(pos, is_white)
    }

    pub fn _get_pawn_move_mask(&self, pos: u64, is_white: bool) -> u64 {
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

    pub fn _get_king_move_mask(&self, pos: u64) -> u64 {
        let mut mask: u64 = 0;

        // Replace with precomputed values to improve performance
    
        // Generate moves to the left
        if pos % 8 > 0 {
            mask |= 1 << (pos - 1);
            if pos / 8 > 0 {
                mask |= 1 << (pos - 9);
            }
            if pos / 8 < 7 {
                mask |= 1 << (pos + 7);
            }
        }
        // Generate moves to the right
        if pos % 8 < 7 {
            mask |= 1 << (pos + 1);
            if pos / 8 > 0 {
                mask |= 1 << (pos - 7);
            }
            if pos / 8 < 7 {
                mask |= 1 << (pos + 9);
            }
        }
        // Generate moves upwards
        if pos / 8 > 0 {
            mask |= 1 << (pos - 8);
        }
        // Generate moves downwards
        if pos / 8 < 7 {
            mask |= 1 << (pos + 8);
        }

        mask

    }    

    pub fn _display_board(&self) -> String {
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

#[allow(dead_code)]
pub fn find_set_bits_positions(mut num: u64) -> Vec<u64> {
    let mut positions = Vec::new();
    let mut bit_position = 1u64;

    while num != 0 {
        if num & 1 == 1 {
            positions.push(bit_position-1);
        }
        num >>= 1;
        bit_position += 1;
    }

    positions
}



// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attack_mask_white_no_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_attack_mask(54, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_white_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_attack_mask(55, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_black_no_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_attack_mask(9, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_black_corner() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_attack_mask(8, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_attack_mask(36, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_pawn_move_mask_white() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_pawn_move_mask(55, true);

        assert_eq!(result, 141287244169216);
    }

    #[test]
    fn test_get_pawn_move_mask_black() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_pawn_move_mask(15, false);

        assert_eq!(result, 2155872256);
    }

    #[test]
    fn test_get_pawn_move_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_pawn_move_mask(36, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_move_white_pawn_1_return_value() {
        let mut chessboard = Chessboard::new();
        let passed = chessboard._move_piece(55, 47, true, false);

        assert!(passed);
    }

    #[test]
    fn test_move_white_pawn_1_board_value() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(55, 47, true, false);

        assert_eq!(chessboard.white_pawn, 35888059530608640);
    }

    #[test]
    fn test_move_black_pawn_2_board_value() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(9, 25, false, false);

        assert_eq!(chessboard.black_pawn, 33619200);
    }

    #[test]
    fn test_move_black_pawn_2_return_value_fail() {
        let mut chessboard = Chessboard::new();
        let passed = chessboard._move_piece(9, 26, false, false);

        assert!(!passed);
    }

    #[test]
    fn test_white_pawn_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(48, 32, true, false);
        chessboard._move_piece(32, 24, true, false);
        chessboard._move_piece(24, 16, true, false);
        chessboard._move_piece(16, 9, true, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18446181123756195839);
    }

    #[test]
    fn test_black_pawn_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(9,25, false, false);
        chessboard._move_piece(25, 33, false, false);
        chessboard._move_piece(33, 41, false, false);
        chessboard._move_piece(41, 48, false, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598732905983);
    }

    #[test]
    fn test_pawn_capture_collision() {
        let mut chessboard = Chessboard::new();
        
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(11, 27, false, false);
        // test if can move forwards if occupied
        chessboard._move_piece(51, 27, true, false);
        chessboard._move_piece(12, 28, false, false);
        // we capture with white
        chessboard._move_piece(35, 28, true, false);
        // we capture with black
        chessboard._move_piece(21, 28, false, false);
    
        
        assert_eq!(chessboard._get_all_piece_mask(), 18444210799321868287);
    }

    #[test]
    fn test_pawn_jump_not_allowed() {
        let mut chessboard = Chessboard::new();
        
        // bring white pawn to front of black pieces
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(35, 27, true, false);
        chessboard._move_piece(27, 19, true, false);
        // bring black pawn to front of white pieces
        chessboard._move_piece(8, 24, false, false);
        chessboard._move_piece(24, 32, false, false);
        chessboard._move_piece(32, 40, false, false);

        //JUMP!
        chessboard._move_piece(48, 32, true, false);
        chessboard._move_piece(11, 27, false, false);

        assert_eq!(chessboard._get_all_piece_mask(), 18444211898431373055);
    }

    #[test]
    fn test_en_passant_square() {
        let mut chessboard = Chessboard::new();
        // create square
        chessboard._move_piece(48, 32, true, false);
        assert_eq!(chessboard.en_passant_square, (1u64 << 40));
        // make sure it's gone
        chessboard._move_piece(32, 24, true, false);
        assert_eq!(chessboard.en_passant_square, 0);
    }

    #[test]
    fn test_en_passant_allowed() {
        let mut chessboard = Chessboard::new();
        // bring white pawn to front of black pieces
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(35, 27, true, false);
        
        // bring black pawn to front of white pieces
        chessboard._move_piece(8, 24, false, false);
        chessboard._move_piece(24, 32, false, false);

        //JUMP!
        chessboard._move_piece(49, 33, true, false);
        // eat
        let epb: bool =chessboard._move_piece(32, 41, false, false);
        //JUMP!
        chessboard._move_piece(12, 28, false, false);
        //eat
        let epw =chessboard._move_piece(27, 20, true, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18443650047990099711);
        assert!(epb);
        assert!(epw);
    }

    #[test]
    fn test_white_knight_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(57, 40, true, false);
        chessboard._move_piece(40, 25, true, false);
        chessboard._move_piece(25, 8, true, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18302347410657050623)
    }

    #[test]
    fn test_black_knight_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(1,18, false, false);
        chessboard._move_piece(18, 35, false, false);
        chessboard._move_piece(35, 52, false, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598732906493)
    }

    #[test]
    fn test_get_knight_move_masks() {
        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(0);
        assert_eq!(result, 132096);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(1);
        assert_eq!(result, 329728);
        
        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(2);
        assert_eq!(result, 659712);
    
        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(6);
        assert_eq!(result, 10489856);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(7);
        assert_eq!(result, 4202496);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(14);
        assert_eq!(result, 2685403152);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(15);
        assert_eq!(result, 1075839008);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(24);
        assert_eq!(result, 2216203387392);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(31);
        assert_eq!(result, 70506185244672);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(32);
        assert_eq!(result, 567348067172352);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(39);
        assert_eq!(result, 18049583422636032);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(48);
        assert_eq!(result, 288234782788157440);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(49);
        assert_eq!(result, 576469569871282176);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(54);
        assert_eq!(result, 1152939783987658752);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(55);
        assert_eq!(result, 2305878468463689728);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(56);
        assert_eq!(result, 1128098930098176);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(57);
        assert_eq!(result, 2257297371824128);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(58);
        assert_eq!(result, 4796069720358912);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(62);
        assert_eq!(result, 4679521487814656);

        let chessboard = Chessboard::new();
        let result = chessboard._get_knight_move_mask(63);
        assert_eq!(result, 9077567998918656);
    }

    #[test]
    fn test_rook_move_mask() {
        let mut chessboard = Chessboard::new();
        chessboard._get_pawn_move_mask(55, true);
        chessboard._get_pawn_move_mask(15, false);
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(35, 27, true, false);
        chessboard._move_piece(8, 24, false, false);
        chessboard._move_piece(24, 32, false, false);
        chessboard._move_piece(49, 33, true, false);
        chessboard._move_piece(32, 41, false, false);
        chessboard._move_piece(12, 28, false, false);
        chessboard._move_piece(27, 20, true, false);
        let result = chessboard._get_rook_move_mask(47, true);
        assert_eq!(result, 139090376818688);
    }

    #[test]
    fn test_white_rook_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(35, 27, true, false);
        chessboard._move_piece(8, 24, false, false);
        chessboard._move_piece(24, 32, false, false);
        chessboard._move_piece(49, 33, true, false);
        chessboard._move_piece(32, 41, false, false);
        chessboard._move_piece(12, 28, false, false);
        chessboard._move_piece(27, 20, true, false);
        chessboard._move_piece(55, 39, true, false);
        chessboard._move_piece(63, 47, true, false);
        chessboard._move_piece(47, 41, true, false);
    
        assert_eq!(chessboard._get_all_piece_mask(), 9184249763872173823);
    }

    #[test]
    fn test_black_rook_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(8, 24, false, false);
        chessboard._move_piece(0, 16, false, false);
        chessboard._move_piece(16, 23, false, false);
        chessboard._move_piece(23, 55, false, false);
    
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598749683454);
    }

    #[test]
    fn test_white_bishop_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(49, 41, true, false);
        chessboard._move_piece(58, 40, true, false);
        chessboard._move_piece(40, 12, true, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18157671471651028991);
    }

    #[test]
    fn test_black_bishop_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(49, 41, true, false);
        chessboard._move_piece(58, 40, true, false);
        chessboard._move_piece(40, 12, true, false);
        chessboard._move_piece(5, 12, false, false);
        assert_eq!(chessboard._get_all_piece_mask(), 18157671471651028959);
    }

    #[test]
    fn test_queen_movement_capture() {
        let mut chessboard = Chessboard::new();
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(60, 24, true, false);
        chessboard._move_piece(24, 10, true, false);
        chessboard._move_piece(10, 1, true, false);
        chessboard._move_piece(1, 2, true, false);
        chessboard._move_piece(2, 11, true, false);
        chessboard._move_piece(4, 11, false, false);
        assert_eq!(chessboard._get_all_piece_mask(), 17291289328672111593)
    }

    #[test]
    fn test_threat_mask() {
        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(12, 28, false, false);
        chessboard._move_piece(13, 29, false, false);
        chessboard._move_piece(14, 30, false, false);
        chessboard._move_piece(15, 31, false, false);
        chessboard._move_piece(1, 18, false, false);
        chessboard._move_piece(0, 16, false, false);
        chessboard._move_piece(2, 17, false, false);
        chessboard._move_piece(3, 19, false, false);
        chessboard._move_piece(5, 33, false, false);
    
        chessboard._move_piece(52, 36, true, false);
        chessboard._move_piece(53, 37, true, false);
        chessboard._move_piece(54, 38, true, false);
        chessboard._move_piece(55, 39, true, false);
        chessboard._move_piece(61, 44, true, false);
        chessboard._move_piece(63, 46, true, false);
        chessboard._move_piece(62, 45, true, false);
        chessboard._move_piece(60, 47, true, false);
        chessboard._move_piece(51, 35, true, false);
        chessboard._move_piece(45, 28, true, false);
        chessboard._move_piece(39, 30, true, false);
    
        assert_eq!(format!("{:?}", chessboard._get_threat_masks(false)),"[343597383680, 1152921504606846976, 34628173824, 0, 268435456, 0]");
        assert_eq!(format!("{:?}", chessboard._get_threat_masks(true)),"[2684354560, 0, 264192, 2147483648, 8589934592, 0]");
    }

    #[test]
    fn test_discovered_check_black() {
        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(11, 27, false, true);
        println!("lol");
        chessboard._move_piece(52, 36, true, true);
        chessboard._move_piece(60, 44, true, true);
        chessboard._move_piece(44, 43, true, true);
        let truval = chessboard._move_piece(27, 36, false, true);    

        assert!(!truval);
    }

    #[test]
    fn test_check_and_discovered_check_white() {
        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(51, 35, true, true);   // White moves
        chessboard._move_piece(12, 28, false, true);    // Black moves
        chessboard._move_piece(4,20, false, true);    // Black moves
        chessboard._move_piece(20, 19, false, true);    // Black moves
        let truval = chessboard._move_piece(35, 28, true, true); // White moves
        assert!(!truval);
        chessboard._move_piece(19, 35, false, true);    // Black moves
        let truval = chessboard._move_piece(60, 51, true, true); // White saves with queen    
        assert!(truval);
    }

    #[test]
    fn test_white_king_movement_check() {
        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(51, 35, true, true);   // White moves
        chessboard._move_piece(59, 51, true, true);   // White moves
        chessboard._move_piece(51, 43, true, true);   // White moves
        chessboard._move_piece(43, 34, true, true);   // White moves
        chessboard._move_piece(34, 27, true, true);   // White moves
        let truval = chessboard._move_piece(27, 19, true, true);   // White moves
        assert!(!truval);
        chessboard._move_piece(6, 21, false, true);   // Black checks
        let truval = chessboard._move_piece(48, 40, true, true);   // White attempts to move pawns but can't
        assert!(!truval);
    }

    #[test]
    fn test_black_king_movement_check() {
        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(11, 27, false, true);   // Black moves
        chessboard._move_piece(3, 11, false, true);   // Black moves
        chessboard._move_piece(11, 19, false, true);   // Black moves
        chessboard._move_piece(19, 26, false, true);   // Black moves
        chessboard._move_piece(26, 34, false, true);   // Black moves
        let truval = chessboard._move_piece(34, 42, false, true);   // Black moves
        assert!(!truval);
        chessboard._move_piece(49, 41, true, true);   // White checks
        let truval = chessboard._move_piece(8, 16, false, true);   // Black attempts to move pawns but can't
        assert!(!truval);
    }

    #[test]
    fn test_pawn_promote_always_to_queen() {
        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(51, 35, true, true);   // White moves
        chessboard._move_piece(35, 27, true, true);   // White moves
        chessboard._move_piece(27, 19, true, true);   // White moves
        chessboard._move_piece(19, 10, true, true);   // White moves
        chessboard._move_piece(10, 1, true, true);   // White moves
        assert_eq!(chessboard._get_all_piece_mask(), 18444210798919220223);

        let mut chessboard: Chessboard = Chessboard::new();
        chessboard._move_piece(12, 28, false, true);   // Black moves
        chessboard._move_piece(28, 36, false, true);   // Black moves
        chessboard._move_piece(36, 44, false, true);   // Black moves
        chessboard._move_piece(44, 51, false, true);   // Black moves
        chessboard._move_piece(51, 58, false, true);   // Black moves
        assert_eq!(chessboard._get_all_piece_mask(), 18446462598732902399);
    }

    #[test]
    fn test_get_board_states_initial_state() {
        let chessboard = Chessboard::new();
        assert_eq!(20, chessboard._get_all_possible_moves(true).len());
        assert_eq!(20, chessboard._get_all_possible_moves(false).len());
    }



}