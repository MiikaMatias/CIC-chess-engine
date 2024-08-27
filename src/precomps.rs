use std::collections::HashMap;
use crate::precomps_knight_logic::*;
use crate::precomps_rook_logic::*;

pub const BISHOP_MOVE_TABLE_SIZE: usize = 5248;


#[derive(Clone)]
pub struct Precomps {
    knight_table: HashMap<u64, u64>,

    rook_entries: [MagicEntry; 64],
    rook_table: Vec<Vec<u64>>,
}

impl Precomps {
    pub fn new() -> Precomps {
        let (rook_entries, rook_table) = init_rook_magics();
        Precomps {
            knight_table: init_knight_and_masks(),
            rook_entries,
            rook_table,
        }
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        return self.knight_table.get(&pos).unwrap().clone();
    }

    pub fn get_rook_move_mask(&self, pos: u64, blockers :u64) -> u64 {
        return self.rook_table[pos as usize][magic_index(&self.rook_entries[pos as usize], blockers) as usize]
    }
}
