use crate::precomps;
use crate::masks::*;
use crate::graphics::*;

#[derive(Copy, Clone, Debug)]
pub struct Chessboard {
    pub pawn: u64, 
    pub rook: u64, 
    pub knight: u64, 
    pub bishop: u64, 
    pub queen: u64,
    pub king: u64, 

    pub white_pieces: u64,
    pub black_pieces: u64,

    pub en_passant_square: u64,

    pub last_captured:u8,
    pub last_capturee:u8,

    pub is_white: bool,

    pub precomps: &'static precomps::Precomps,
}

impl PartialEq for Chessboard {
    fn eq(&self, other: &Self) -> bool {
        return (self.pawn != other.pawn) | (self.rook != other.rook) | (self.knight != other.knight) | (self.bishop != other.bishop) | (self.queen != other.queen) | (self.king != other.king);
    }
}

impl Chessboard {
    pub fn new(precomps: &'static precomps::Precomps) -> Chessboard {
        Chessboard {
            pawn: 71776119061217280 | 65280, // White pawns have a larger number; "bottom"
            rook: 9295429630892703744 | 129, 
            knight: 4755801206503243776 | 66, 
            bishop: 2594073385365405696 | 36, 
            queen: 576460752303423488 | 8,
            king: 1152921504606846976 | 16, 

            white_pieces: 71776119061217280 | 9295429630892703744 | 4755801206503243776 | 2594073385365405696 | 576460752303423488 | 1152921504606846976,
            black_pieces: 65280 | 129 | 66 | 36 | 8 | 16,

            en_passant_square: 0,

            last_captured: 0,
            last_capturee: 0,

            is_white: true,

            precomps: precomps
        }
    }

    pub fn get_hash(&self, depth:u64) -> u64 {
        self.pawn.wrapping_mul(self.precomps.pawn_hash)
            .wrapping_add(self.rook.wrapping_mul(self.precomps.rook_hash))
            .wrapping_add(self.knight.wrapping_mul(self.precomps.knight_hash))
            .wrapping_add(self.bishop.wrapping_mul(self.precomps.bishop_hash))
            .wrapping_add(self.queen.wrapping_mul(self.precomps.queen_hash))
            .wrapping_add(self.king.wrapping_mul(self.precomps.king_hash)).wrapping_add(depth.wrapping_mul(self.precomps.turn_hash))
    }
    
    pub fn get_pawn_move_mask(&self, pos: u64) -> u64 {
        if self.pawn | (1u64 << pos) != self.pawn {
            return 0
        }

        let (moves, attacks) = self.precomps.get_pawn_move_mask(pos, self.is_white);
        let all_pieces = self.get_all_pieces();

        if self.is_white {
            let move_decider = all_pieces | ((all_pieces & !(1u64 << pos)) >> 8);
            let preliminary = !move_decider & moves & !all_pieces;

            return preliminary | (attacks & self.black_pieces)

        } else {
            let move_decider = all_pieces | ((all_pieces & !(1u64 << pos)) << 8);
            let preliminary = !move_decider & moves & !all_pieces;

            return preliminary | (attacks & self.white_pieces) 
        }
    }  

    pub fn get_queen_move_mask(&self, pos: u64) -> u64 {
        self.precomps.get_rook_move_mask(pos, self.get_all_pieces()) | self.precomps.get_bishop_move_mask(pos, self.get_all_pieces())
    }

    pub fn get_all_moves_at_position(&self, pos: u64) -> Vec<u64> {
        let white_mask = -(self.is_white as i64) as u64;
        let black_mask = !white_mask; 
        
        let pieces = (self.white_pieces & white_mask) | (self.black_pieces & black_mask);
        let all_pieces = self.get_all_pieces();
        
        let pos_mask = 1u64 << pos;
        let empty_squares = !pieces;
    
        let pawn_moves = (self.get_pawn_move_mask(pos) & empty_squares) * ((self.pawn & pos_mask) >> pos);
        let knight_moves = (self.precomps.get_knight_move_mask(pos) & empty_squares) * ((self.knight & pos_mask) >> pos);
        let bishop_moves = (self.precomps.get_bishop_move_mask(pos, all_pieces) & empty_squares) * ((self.bishop & pos_mask) >> pos);
        let rook_moves = (self.precomps.get_rook_move_mask(pos, all_pieces) & empty_squares) * ((self.rook & pos_mask) >> pos);
        let king_moves = (get_king_move_mask(pos) & empty_squares) * ((self.king & pos_mask) >> pos);
        let queen_moves = (self.get_queen_move_mask(pos) & empty_squares) * (((!(self.pawn | self.knight | self.bishop | self.rook | self.king) & pos_mask) >> pos));
        
        let all_moves = (pawn_moves | knight_moves | bishop_moves | rook_moves | king_moves | queen_moves) * (all_pieces & pos_mask).min(1);
        
        find_set_bits_positions(all_moves)
    }

    // always returns only one sides' moves
    pub fn get_all_possible_moves(&self) -> Vec<Chessboard> {
        let mut board_array: Vec<Chessboard> = Vec::new();
        let _i = 0;

        let pieces =  [self.get_white_pawns(), self.get_white_bishops(), self.get_white_kings(), self.get_white_queens(), self.get_white_rooks(), self.get_white_knights(), 
        self.get_black_pawns(), self.get_black_bishops(), self.get_black_kings(), self.get_black_queens(), self.get_black_rooks(), self.get_black_knights()];

        let increment = 6 * !self.is_white as u64;

        for i in 0..6 {
            let positions_of_pieces = find_set_bits_positions(pieces[i+increment as usize]);
            for position in positions_of_pieces {
                let moves_of_position = self.get_all_moves_at_position(position);
                for move_target in moves_of_position {
                    let mut new_chessboard = *self;
                    let is_legal = new_chessboard.move_piece(position, move_target);
                    if is_legal {
                        board_array.push(new_chessboard);
                    }
                }
            }
        }

        board_array
    }

    pub fn get_threat_masks(&self, is_white: bool) -> [u64; 6] {
        let mut threat_mask: [u64; 6] = [0; 6];
    
        let (pawns, knights, bishops, rooks, queens, kings) = if is_white {
            (
                self.get_white_pawns(),
                self.get_white_knights(),
                self.get_white_bishops(),
                self.get_white_rooks(),
                self.get_white_queens(),
                self.get_white_kings(),
            )
        } else {
            (
                self.get_black_pawns(),
                self.get_black_knights(),
                self.get_black_bishops(),
                self.get_black_rooks(),
                self.get_black_queens(),
                self.get_black_kings(),
            )
        };
    
        fn process_threats(bitboard: u64, threat_mask: &mut u64, is_white: bool, chessboard: &Chessboard) {
            let mut pieces = bitboard;
            while pieces != 0 {
                let check_square = pieces.trailing_zeros() as u64;
                *threat_mask |= chessboard.get_attack_mask(check_square, is_white);
                pieces &= pieces - 1; 
            }
        }
    
        process_threats(pawns, &mut threat_mask[0], is_white, self);
        process_threats(knights, &mut threat_mask[1], is_white, self);
        process_threats(bishops, &mut threat_mask[2], is_white, self);
        process_threats(rooks, &mut threat_mask[3], is_white, self);
        process_threats(queens, &mut threat_mask[4], is_white, self);
        process_threats(kings, &mut threat_mask[5], is_white, self);
    
        threat_mask
    }
    
    pub fn threatened_squares(&self, is_white: bool) -> u64 {
        return self.get_threat_masks(is_white).iter().cloned().fold(0, |acc, x| acc | x);
    }

    pub fn get_all_pieces(&self) -> u64 {
        return self.white_pieces | self.black_pieces;
    }

    pub fn get_white_pieces(&self) -> u64 {
        return self.get_all_pieces() & self.white_pieces;
    }

    pub fn get_black_pieces(&self) -> u64 {
        return self.get_all_pieces() & self.black_pieces;
    }

    pub fn get_white_pawns(&self) -> u64 {
        return self.pawn & self.white_pieces;
    }

    pub fn get_white_knights(&self) -> u64 {
        return self.knight & self.white_pieces;
    }

    pub fn get_white_rooks(&self) -> u64 {
        return self.rook & self.white_pieces;
    }

    pub fn get_white_bishops(&self) -> u64 {
        return self.bishop & self.white_pieces;
    }

    pub fn get_white_queens(&self) -> u64 {
        return self.queen & self.white_pieces;
    }

    pub fn get_white_kings(&self) -> u64 {
        return self.king & self.white_pieces;
    }

    pub fn get_black_pawns(&self) -> u64 {
        return self.pawn & self.black_pieces;
    }

    pub fn get_black_knights(&self) -> u64 {
        return self.knight & self.black_pieces;
    }

    pub fn get_black_rooks(&self) -> u64 {
        return self.rook & self.black_pieces;
    }

    pub fn get_black_bishops(&self) -> u64 {
        return self.bishop & self.black_pieces;
    }

    pub fn get_black_queens(&self) -> u64 {
        return self.queen & self.black_pieces;
    }

    pub fn get_black_kings(&self) -> u64 {
        return self.king & self.black_pieces;
    }

    pub fn _self_check_check(&self, from: u64, to: u64) -> bool {
        let mut next_state = *self;
        if next_state.pawn | (1u64 << from) == next_state.pawn {
            next_state.move_pawn(from, to);
        } else if next_state.knight | (1u64 << from) == next_state.knight {
            next_state.move_knight(from, to);
        } else if next_state.bishop | (1u64 << from) == next_state.bishop {
            next_state.move_bishop(from, to);
        } else if next_state.rook | (1u64 << from) == next_state.rook {
            next_state.move_rook(from, to);
        } else if next_state.queen | (1u64 << from) == next_state.queen {
            next_state.move_queen(from, to);
        } else if next_state.king | (1u64 << from) == next_state.king {
            next_state.move_king(from, to);
        }
        next_state.is_white = !next_state.is_white;
        is_check(next_state, !next_state.is_white)
    }

    pub fn check_en_passant(&self, pos: u64, return_mask: u64) -> u64 {
        if self.en_passant_square as i64 - pos as i64 == 7 || 
            self.en_passant_square as i64 - pos as i64 == 9 ||
            pos as i64 - self.en_passant_square as i64 == 7 ||
            pos as i64 - self.en_passant_square as i64 == 9 {
            return return_mask | self.en_passant_square;
        }
        return_mask
    }

    pub fn get_piece_type(&self, pos: u64) -> u8 {
        if self.pawn | (1u64 << pos) == self.pawn {
            return 1;
        } else if self.knight | (1u64 << pos) == self.knight {
            return 2;
        } else if self.bishop | (1u64 << pos) == self.bishop {
            return 3;
        } else if self.rook | (1u64 << pos) == self.rook {
            return 4;
        } else if self.queen | (1u64 << pos) == self.queen {
            return 5;
        } else if self.king | (1u64 << pos) == self.king {
            return 6;
        } else {
            return 0;
        }
    }

    pub fn move_piece(&mut self, from: u64, to: u64) -> bool {

        if self._self_check_check(from, to) {
            return false;
        }

        self.last_captured = self.get_piece_type(to);

        if self.last_captured == 1 {
            self.pawn &= !(1u64 << to);
        } else if self.last_captured == 2 {
            self.knight &= !(1u64 << to);
        } else if self.last_captured == 3 {
            self.bishop &= !(1u64 << to);
        } else if self.last_captured == 4 {
            self.rook &= !(1u64 << to);
        } else if self.last_captured == 5 {
            self.queen &= !(1u64 << to);
        } else if self.last_captured == 6 {
            self.king &= !(1u64 << to);
        }
        
        if self.pawn | (1u64 << from) == self.pawn {
            self.move_pawn(from, to);
        } else if self.knight | (1u64 << from) == self.knight {
            self.move_knight(from, to);
        } else if self.bishop | (1u64 << from) == self.bishop {
            self.move_bishop(from, to);
        } else if self.rook | (1u64 << from) == self.rook {
            self.move_rook(from, to);
        } else if self.queen | (1u64 << from) == self.queen {
            self.move_queen(from, to);
        } else if self.king | (1u64 << from) == self.king {
            self.move_king(from, to);
        }
        self.is_white = !self.is_white;
        return true
    }

    pub fn move_pawn(&mut self, from: u64, to: u64) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;
    
        self.pawn &= !(from_mask);
        if ((to_mask) | RANK_8_MASK) == RANK_8_MASK || ((to_mask) | RANK_1_MASK) == RANK_1_MASK {
            self.queen |= to_mask;
        } else {
            self.pawn |= to_mask;
        }

        if (self.white_pieces & from_mask) != 0 {
            if (self.black_pieces & to_mask) != 0 {
                self.last_capturee = 1
            }
            self.white_pieces &= !from_mask;
            self.white_pieces |= to_mask;
            self.black_pieces &= !to_mask;
        } else if (self.black_pieces & from_mask) != 0 {
            if (self.white_pieces & to_mask) != 0 {
                self.last_capturee = 1
            }
            self.black_pieces &= !from_mask;
            self.black_pieces |= to_mask;
            self.white_pieces &= !to_mask;
        }
    
        if to_mask == self.en_passant_square {
            if (self.white_pieces & to_mask) != 0 {
                self.black_pieces &= !(to_mask >> 8);
                self.pawn &= !(to_mask << 8);
            } else if (self.black_pieces & to_mask) != 0 {
                self.white_pieces &= !(to_mask << 8);
                self.pawn &= !(to_mask >> 8);
            }
        }
    
        if to > from && (to - from == 16) {
            self.en_passant_square = to_mask >> 8; 
        } else if from > to && (from - to == 16) { 
            self.en_passant_square = to_mask << 8; 
        } else {
            self.en_passant_square = 0;
        }
    }

    pub fn move_knight(&mut self, from: u64, to: u64) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.knight &= !from_mask;
        self.knight |= to_mask;
        if (self.white_pieces & from_mask) != 0 {
            if (self.black_pieces & to_mask) != 0 {
                self.last_capturee = 2
            }
            self.white_pieces &= !from_mask;
            self.white_pieces |= to_mask;
            self.black_pieces &= !(to_mask);
        } else if (self.black_pieces & from_mask) != 0 {
            if (self.white_pieces & to_mask) != 0 {
                self.last_capturee = 2
            }
            self.black_pieces &= !from_mask;
            self.black_pieces |= to_mask;
            self.white_pieces &= !(to_mask);
        }
    }

    pub fn move_bishop(&mut self, from: u64, to: u64) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.bishop &= !from_mask;
        self.bishop |= to_mask;
        if (self.white_pieces & from_mask) != 0 {
            if (self.black_pieces & to_mask) != 0 {
                self.last_capturee = 3
            }
            self.white_pieces &= !from_mask;
            self.white_pieces |= to_mask;
            self.black_pieces &= !(to_mask);
        } else if (self.black_pieces & from_mask) != 0 {
            if (self.white_pieces & to_mask) != 0 {
                self.last_capturee = 3
            }
            self.black_pieces &= !from_mask;
            self.black_pieces |= to_mask;
            self.white_pieces &= !(to_mask);
        }
    }

    pub fn move_rook(&mut self, from: u64, to: u64) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.rook &= !from_mask;
        self.rook |= to_mask;
        if (self.white_pieces & from_mask) != 0 {
            if (self.black_pieces & to_mask) != 0 {
                self.last_capturee = 4
            }
            self.white_pieces &= !from_mask;
            self.white_pieces |= to_mask;
            self.black_pieces &= !(to_mask);
        } else if (self.black_pieces & from_mask) != 0 {
            if (self.white_pieces & to_mask) != 0 {
                self.last_capturee = 4
            }
            self.black_pieces &= !from_mask;
            self.black_pieces |= to_mask;
            self.white_pieces &= !(to_mask);
        }
    }

    pub fn move_queen(&mut self, from: u64, to: u64) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.queen &= !from_mask;
        self.queen |= to_mask;
        if (self.white_pieces & from_mask) != 0 {
            if (self.black_pieces & to_mask) != 0 {
                self.last_capturee = 5
            }
            self.white_pieces &= !from_mask;
            self.white_pieces |= to_mask;
            self.black_pieces &= !(to_mask);
        } else if (self.black_pieces & from_mask) != 0 {
            if (self.white_pieces & to_mask) != 0 {
                self.last_capturee = 5
            }
            self.black_pieces &= !from_mask;
            self.black_pieces |= to_mask;
            self.white_pieces &= !(to_mask);
        }
    }
    pub fn move_king(&mut self, from: u64, to: u64) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        self.king &= !from_mask;
        self.king |= to_mask;
        if (self.white_pieces & from_mask) != 0 {
            if (self.black_pieces & to_mask) != 0 {
                self.last_capturee = 6
            }
            self.white_pieces &= !from_mask;
            self.white_pieces |= to_mask;
            self.black_pieces &= !(to_mask);
        } else if (self.black_pieces & from_mask) != 0 {
            self.black_pieces &= !from_mask;
            self.black_pieces |= to_mask;
            self.white_pieces &= !(to_mask);
        }
    }
    
    pub fn get_attack_mask(&self, pos: u64, is_white: bool) -> u64 {
        if is_white {
            if ((self.get_white_pawns() >> pos) & 1u64) == 1 {
                if (1u64 << pos | FILE_H_MASK) == FILE_H_MASK {
                    return self.check_en_passant(pos, (1u64 << (pos-9)) & self.black_pieces);
                } else if (1u64 << pos | FILE_A_MASK) == FILE_A_MASK {
                    return  self.check_en_passant(pos, (1u64 << (pos-7)) & self.black_pieces);
                } else {
                    return  self.check_en_passant(pos, ((1u64 << (pos-9))|(1u64 << (pos-7))) & self.black_pieces);
                }
            } else if ((self.get_white_knights() >> pos) & 1u64) == 1 {   
                return self.precomps.get_knight_move_mask(pos) & self.black_pieces
            } else if ((self.get_white_rooks() >> pos) & 1u64) == 1 {   
                return self.precomps.get_rook_move_mask(pos, self.get_all_pieces()) & self.black_pieces
            } else if ((self.get_white_bishops() >> pos) & 1u64) == 1 {   
                return self.precomps.get_bishop_move_mask(pos, self.get_all_pieces()) & self.black_pieces
            } else if ((self.get_white_queens() >> pos) & 1u64) == 1 {   
                return self.get_queen_move_mask(pos) & self.black_pieces
            } else if ((self.get_white_kings() >> pos) & 1u64) == 1 {   
                return get_king_move_mask(pos) & self.black_pieces
            } 
            return 0;
        }
        if ((self.get_black_pawns() >> pos) & 1u64) == 1 {
            if (1u64 << pos | FILE_H_MASK) == FILE_H_MASK  {
                return  self.check_en_passant(pos, (1u64 << (pos+7)) & self.white_pieces);
            } else if (1u64 << pos | FILE_A_MASK) == FILE_A_MASK {
                return  self.check_en_passant(pos, (1u64 << (pos+9)) & self.white_pieces);
            } else {
                return  self.check_en_passant(pos, ((1u64 << (pos+9))|(1u64 << (pos+7))) & self.white_pieces);
            }
        } else if ((self.get_black_knights() >> pos) & 1u64) == 1 {   
            return self.precomps.get_knight_move_mask(pos) & self.white_pieces
        } else if ((self.get_black_rooks() >> pos) & 1u64) == 1 {   
            return self.precomps.get_rook_move_mask(pos, self.get_all_pieces()) & self.white_pieces
        } else if ((self.get_black_bishops() >> pos) & 1u64) == 1 {   
            return self.precomps.get_bishop_move_mask(pos, self.get_all_pieces()) & self.white_pieces
        } else if ((self.get_black_queens() >> pos) & 1u64) == 1 {   
            return self.get_queen_move_mask(pos) & self.white_pieces
        } else if ((self.get_black_kings() >> pos) & 1u64) == 1 {   
            return get_king_move_mask(pos) & self.white_pieces
        }
        0
    }

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

pub fn is_check(state: Chessboard, towards_white: bool) -> bool {
    // get squares threatened by the opponent
    let opponent_threatens: u64 = state.threatened_squares(!towards_white);
    if towards_white {
        (state.get_white_kings() & opponent_threatens) == state.get_white_kings()
    } else {
        (state.get_black_kings() & opponent_threatens) == state.get_black_kings()
    }
}


pub fn get_king_move_mask(pos: u64) -> u64 {
    let mut mask: u64 = 0;

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




// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use crate::{graphics::*, uci_wrapper::apply_fen};

    static PRECOMPS: LazyLock<precomps::Precomps> = LazyLock::new(|| precomps::Precomps::new());


    #[test]
    fn test_get_attack_mask_white_no_corner() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_attack_mask(54, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_white_corner() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_attack_mask(55, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_black_no_corner() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_attack_mask(9, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_black_corner() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_attack_mask(8, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_attack_mask_no_pawn() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_attack_mask(36, false);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_pawn_move_mask_white() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_pawn_move_mask(55);

        assert_eq!(result, 141287244169216);
    }

    #[test]
    fn test_get_pawn_move_mask_black() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.is_white = false;
        let result = chessboard.get_pawn_move_mask(15);
        println!("{} ", display_bit_board(result));

        assert_eq!(result, 2155872256);
    }

    #[test]
    fn test_get_pawn_move_mask_no_pawn() {
        let precomps = &PRECOMPS;
        let chessboard = Chessboard::new(&precomps);
        let result = chessboard.get_pawn_move_mask(36);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_move_white_pawn_1_board_value() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(55, 47);
        println!("{} ", display_bit_board(chessboard.get_all_pieces()));

        assert_eq!(chessboard.get_white_pawns(), 35888059530608640);
    }

    #[test]
    fn test_move_black_pawn_2_board_value() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(9, 25);

        assert_eq!(chessboard.get_black_pawns(), 33619200);
    }

    #[test]
    fn test_white_pawn_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(48, 32);
        chessboard.move_piece(32, 24);
        chessboard.move_piece(24, 16);
        chessboard.move_piece(16, 9);
        assert_eq!(chessboard.get_all_pieces(), 18446181123756195839);
    }

    #[test]
    fn test_black_pawn_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(9,25);
        chessboard.move_piece(25, 33);
        chessboard.move_piece(33, 41);
        chessboard.move_piece(41, 48);
        assert_eq!(chessboard.get_all_pieces(), 18446462598732905983);
    }

    #[test]
    fn test_pawn_capture_collision() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        
        chessboard.move_piece(51, 35);
        chessboard.move_piece(11, 27);
        // test if can move forwards if occupied
        chessboard.move_piece(51, 27);
        chessboard.move_piece(12, 28);
        // we capture with white
        chessboard.move_piece(35, 28);
        // we capture with black
        chessboard.move_piece(21, 28);
    
        
        assert_eq!(chessboard.get_all_pieces(), 18444210799321868287);
    }

    #[test]
    fn test_pawn_jump_not_allowed() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        apply_fen(&mut chessboard, "rnbqkbnr/pppppppp/P7/8/8/P7/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
   
        println!("{} ", display_board(&chessboard));
        let m1 = chessboard.get_pawn_move_mask(48);
        assert_eq!(m1, 0);
        chessboard.is_white = false;
        let m2= chessboard.get_pawn_move_mask(8);
        println!("{}\n{}", display_bit_board(m1), display_bit_board(m2));
        assert_eq!(m2, 0);
    }

    #[test]
    fn test_en_passant_square() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        // create square
        chessboard.move_piece(48, 32);
        assert_eq!(chessboard.en_passant_square, (1u64 << 40));
        // make sure it's gone
        chessboard.move_piece(32, 24);
        assert_eq!(chessboard.en_passant_square, 0);
    }

    #[test]
    fn test_en_passant_allowed() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        // bring white pawn to front of black pieces
        chessboard.move_piece(51, 35);
        chessboard.move_piece(35, 27);

        // bring black pawn to front of white pieces
        chessboard.move_piece(8, 24);
        chessboard.move_piece(24, 32);

        //JUMP!
        chessboard.move_piece(49, 33);
        // eat
        let epb: bool =chessboard.move_piece(32, 41);
        //JUMP!
        chessboard.move_piece(12, 28);
        //eat
        let epw =chessboard.move_piece(27, 20);
        assert_eq!(chessboard.get_all_pieces(), 18443650056848469759);
        assert!(epb);
        assert!(epw);
    }

    #[test]
    fn test_white_knight_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(57, 40);
        chessboard.move_piece(40, 25);
        chessboard.move_piece(25, 8);
        assert_eq!(chessboard.get_all_pieces(), 18302347410657050623)
    }

    #[test]
    fn test_black_knight_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(1,18);
        chessboard.move_piece(18, 35);
        chessboard.move_piece(35, 52);
        assert_eq!(chessboard.get_all_pieces(), 18446462598732906493)
    }

    #[test]
    fn get_knight_move_masks() {
        let result = PRECOMPS.get_knight_move_mask(0);
        assert_eq!(result, 132096);

        let result = PRECOMPS.get_knight_move_mask(1);
        assert_eq!(result, 329728);
        
        let result = PRECOMPS.get_knight_move_mask(2);
        assert_eq!(result, 659712);
    
        let result = PRECOMPS.get_knight_move_mask(6);
        assert_eq!(result, 10489856);

        let result = PRECOMPS.get_knight_move_mask(7);
        assert_eq!(result, 4202496);

        let result = PRECOMPS.get_knight_move_mask(14);
        assert_eq!(result, 2685403152);

        let result = PRECOMPS.get_knight_move_mask(15);
        assert_eq!(result, 1075839008);

        let result = PRECOMPS.get_knight_move_mask(24);
        assert_eq!(result, 2216203387392);

        let result = PRECOMPS.get_knight_move_mask(31);
        assert_eq!(result, 70506185244672);

        let result = PRECOMPS.get_knight_move_mask(32);
        assert_eq!(result, 567348067172352);

        let result = PRECOMPS.get_knight_move_mask(39);
        assert_eq!(result, 18049583422636032);

        let result = PRECOMPS.get_knight_move_mask(48);
        assert_eq!(result, 288234782788157440);

        let result = PRECOMPS.get_knight_move_mask(49);
        assert_eq!(result, 576469569871282176);

        let result = PRECOMPS.get_knight_move_mask(54);
        assert_eq!(result, 1152939783987658752);

        let result = PRECOMPS.get_knight_move_mask(55);
        assert_eq!(result, 2305878468463689728);

        let result = PRECOMPS.get_knight_move_mask(56);
        assert_eq!(result, 1128098930098176);

        let result = PRECOMPS.get_knight_move_mask(57);
        assert_eq!(result, 2257297371824128);

        let result = PRECOMPS.get_knight_move_mask(58);
        assert_eq!(result, 4796069720358912);

        let result = PRECOMPS.get_knight_move_mask(62);
        assert_eq!(result, 4679521487814656);

        let result = PRECOMPS.get_knight_move_mask(63);
        assert_eq!(result, 9077567998918656);
    }

    #[test]
    fn test_rook_move_mask() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.get_pawn_move_mask(55);
        chessboard.get_pawn_move_mask(15);
        chessboard.move_piece(51, 35);
        chessboard.move_piece(35, 27);
        chessboard.move_piece(8, 24);
        chessboard.move_piece(24, 32);
        chessboard.move_piece(49, 33);
        chessboard.move_piece(32, 41);
        chessboard.move_piece(12, 28);
        chessboard.move_piece(27, 20);
        let result = chessboard.precomps.get_rook_move_mask(47, chessboard.get_all_pieces());
        assert_eq!(result, 36167887395782656); //36167887395782656
    }

    #[test]
    fn test_white_rook_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(51, 35);
        chessboard.move_piece(35, 27);
        chessboard.move_piece(8, 24);
        chessboard.move_piece(24, 32);
        chessboard.move_piece(49, 33);
        chessboard.move_piece(32, 41);
        chessboard.move_piece(12, 28);
        chessboard.move_piece(27, 20);
        chessboard.move_piece(55, 39);
        chessboard.move_piece(63, 47);
        chessboard.move_piece(47, 41);
    
        assert_eq!(chessboard.get_all_pieces(), 9184249772730543871);
    }

    #[test]
    fn test_black_rook_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(8, 24);
        chessboard.move_piece(0, 16);
        chessboard.move_piece(16, 23);
        chessboard.move_piece(23, 55);
    
        assert_eq!(chessboard.get_all_pieces(), 18446462598749683454);
    }

    #[test]
    fn test_white_bishop_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(49, 41);
        chessboard.move_piece(58, 40);
        chessboard.move_piece(40, 12);
        assert_eq!(chessboard.get_all_pieces(), 18157671471651028991);
    }

    #[test]
    fn test_black_bishop_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        println!("{} ", display_bit_board(chessboard.get_all_pieces()));
        chessboard.move_piece(49, 41);
        chessboard.move_piece(58, 40);
        chessboard.move_piece(40, 12);
        chessboard.move_piece(5, 12);
        println!("{} ", display_bit_board(chessboard.get_all_pieces()));
        assert_eq!(chessboard.get_all_pieces(), 18157671471651028959);
    }

    #[test]
    fn test_queen_movement_capture() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        chessboard.move_piece(51, 35);
        chessboard.move_piece(59, 43);
        chessboard.move_piece(43, 16);
        chessboard.move_piece(16, 9);
        chessboard.move_piece(9, 2);
        chessboard.move_piece(3, 2);
        assert_eq!(chessboard.get_all_pieces(), 17867750080975535607)
    }

    #[test]
    fn test_threat_mask() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(12, 28);
        chessboard.move_piece(13, 29);
        chessboard.move_piece(14, 30);
        chessboard.move_piece(15, 31);
        chessboard.move_piece(1, 18);
        chessboard.move_piece(0, 16);
        chessboard.move_piece(2, 17);
        chessboard.move_piece(3, 19);
        chessboard.move_piece(5, 33);

        chessboard.move_piece(52, 36);
        chessboard.move_piece(53, 37);
        chessboard.move_piece(54, 38);
        chessboard.move_piece(55, 39);
        chessboard.move_piece(61, 44);
        chessboard.move_piece(63, 46);
        chessboard.move_piece(62, 45);
        chessboard.move_piece(60, 47);
        chessboard.move_piece(51, 35);
        chessboard.move_piece(45, 28);
        chessboard.move_piece(39, 30);

        assert_eq!(format!("{:?}", chessboard.get_threat_masks(false)),"[343597383680, 34628173824, 34359738368, 281474976710656, 34628173824, 0]");
        assert_eq!(format!("{:?}", chessboard.get_threat_masks(true)),"[2684354560, 264192, 0, 0, 0, 0]");
    }

    #[test]
    fn test_discovered_check_black() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(11, 27);
        chessboard.move_piece(52, 36);
        chessboard.move_piece(60, 44);
        chessboard.move_piece(44, 43);
        chessboard.move_piece(2, 29);
        println!("{}", display_board(&chessboard));
        chessboard.is_white = true;
        let legal = chessboard.move_piece(36, 27);    
        println!("{}", display_board(&chessboard));
        assert_eq!(legal, false);
    }

    #[test]
    fn test_check_and_discovered_check_white() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(52, 36);   // White moves
        chessboard.move_piece(11, 27);    // Black moves
        chessboard.move_piece(3,19);    // Black moves
        chessboard.move_piece(19, 20);    // Black moves
        let truval = chessboard.move_piece(36, 27); // White moves
        assert!(!truval);
    }

    #[test]
    fn test_white_king_movement_check() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(52, 36);   // White moves
        chessboard.move_piece(60, 52);   // White moves
        chessboard.move_piece(52, 44);   // White moves
        chessboard.move_piece(44, 35);   // White moves
        chessboard.move_piece(35, 28);   // White moves
        chessboard.is_white = true;
        let truval = chessboard.move_piece(28, 20);   // White moves
        assert!(!truval);
        chessboard.is_white = false;
        chessboard.move_piece(11, 19);   // Black checks
        println!("{}", display_board(&chessboard));
        let truval = chessboard.move_piece(48, 40);   // White attempts to move pawns but can't
        println!("{}", display_board(&chessboard));
        assert!(!truval);
    }

    #[test]
    fn test_black_king_movement_check() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(12, 28);   // Black moves
        chessboard.move_piece(4, 12);   // Black moves
        chessboard.move_piece(12, 20);   // Black moves
        chessboard.move_piece(20, 27);   // Black moves
        chessboard.move_piece(27, 35);   // Black moves
        chessboard.is_white = true;
        let truval = chessboard.move_piece(62, 45);   // White moves
        assert!(truval);
        chessboard.move_piece(49, 41);   // White checks
        println!("{}", display_board(&chessboard));
        let truval = chessboard.move_piece(8, 16);   // Black attempts to move pawns but can't
        assert!(!truval);
    }

    #[test]
    fn test_pawn_promote_always_to_queen() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(51, 1);   // White moves
        assert_eq!(chessboard.get_all_pieces(), 18444210798919221247);

        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);
        chessboard.move_piece(12, 58);   // Black moves
        assert_eq!(chessboard.get_all_pieces(), 18446462598732902399);
    }

    #[test]
    fn test_get_board_states_initial_state() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);

        let white_states = chessboard.get_all_possible_moves();
        for state in white_states {
            println!("{}", display_board(&state));
        }
        let white_states = chessboard.get_all_possible_moves();
        assert_eq!(20, white_states.len());
        chessboard.is_white = false;
        let black_states = chessboard.get_all_possible_moves();
        assert_eq!(20, black_states.len());
    }
    #[test]
    fn run_scholars_mate() {
        let precomps = &PRECOMPS;
        let mut chessboard = Chessboard::new(&precomps);
        apply_fen(&mut chessboard, "r1bqkbnr/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4");
        chessboard.is_white = false;
        assert_eq!(0, chessboard.get_all_possible_moves().len());
    }

    #[test]
    fn test_captures() {
        let precomps = &PRECOMPS;
        let mut chessboard: Chessboard = Chessboard::new(precomps);

        chessboard.move_piece(48, 32); 
        chessboard.move_piece(1, 18); 
        chessboard.move_piece(18, 35); 
        chessboard.move_piece(35, 25); 

        assert!(chessboard.move_piece(32, 25)); 
        assert_eq!(chessboard.last_capturee, 1);
        assert_eq!(chessboard.last_captured, 2);
    }

}