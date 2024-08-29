use crate::masks::*;


pub fn precompute_pawn_move(pos: u64, is_white: bool) -> (u64, u64) {
    if pos < 8 || pos >= 56 {
        return (0, 0);
    }
    let mut attacks = 0;
    if is_white {
        if ((1u64 << pos) & RANK_2_MASK) != 0 {
            let mut moves: u64 = 1u64 << (pos - 8);
            if pos >= 48 && pos < 56 { 
                moves |= 1u64 << (pos - 16);
            }
            if (1u64 << pos) & !FILE_A_MASK != 0 {
                attacks |= 1u64 << (pos - 9);
            }
            if (1u64 << pos) & !FILE_H_MASK != 0 {
                attacks |= 1u64 << (pos - 7);
            }
            return (moves, attacks);
        } else {
            let moves = 1u64 << (pos - 8);
            if (1u64 << pos) & !FILE_A_MASK != 0 {
                attacks |= 1u64 << (pos - 9);
            }
            if (1u64 << pos) & !FILE_H_MASK != 0 {
                attacks |= 1u64 << (pos - 7);
            }
            return (moves, attacks);
        }
    } else {
        if ((1u64 << pos) & RANK_7_MASK) != 0 {
            let mut moves = 1u64 << (pos + 8);
            if pos >= 8 && pos < 16 { 
                moves |= 1u64 << (pos + 16);
            }
            if (1u64 << pos) & !FILE_A_MASK != 0 {
                attacks |= 1u64 << (pos + 7);
            }
            if (1u64 << pos) & !FILE_H_MASK != 0 {
                attacks |= 1u64 << (pos + 9);
            }
            return (moves, attacks);
        } else {
            let moves = 1u64 << (pos + 8);
            if (1u64 << pos) & !FILE_A_MASK != 0 {
                attacks |= 1u64 << (pos + 7);
            }
            if (1u64 << pos) & !FILE_H_MASK != 0 {
                attacks |= 1u64 << (pos + 9);
            }
            return (moves, attacks);
        }
    }
}

// Black offset is 64
pub fn init_pawn_and_masks() -> [(u64, u64); 128] {
    let mut pawn_table: [(u64, u64); 128] = [(0,0); 128];
    for pos in 0..64 {
        pawn_table[pos] = precompute_pawn_move(pos as u64, true);
    }
    for pos in 0..64 {
        pawn_table[pos+64] = precompute_pawn_move(pos as u64, false);
    }

    return pawn_table;
}