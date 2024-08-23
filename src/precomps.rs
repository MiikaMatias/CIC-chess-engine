use std::collections::HashMap;
use crate::precomps_bishop_logic::*;
use crate::precomps_knight_logic::*;
use crate::precomps_rook_logic::*;

pub const BISHOP_MOVE_TABLE_SIZE: usize = 5248;


#[derive(Clone)]
pub struct Precomps {
    knight_table: HashMap<u64, u64>,

    rook_magics: Vec<u64>,
    rook_moves: HashMap<u64, Vec<u64>>,

    bishop_magics: Vec<u64>,
}

impl Precomps {
    pub fn new() -> Precomps {
        Precomps {
            knight_table: init_knight_and_masks(),
            
            rook_moves: init_rook_and_attack_map(),
            rook_magics: init_rook_magics(),

            bishop_magics: init_bishop_magics(),
        }
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        return self.knight_table.get(&pos).unwrap().clone();
    }

    pub fn get_rook_move_mask(&self, square: u64, blockers: u64) -> u64 {
        return 0; // moves needs to be pre-generated
    }

    pub fn get_bishop_move_mask(&self, square: u64, blockers: u64) -> u64 {
        return 0 // moves needs to be pre-generated
    }
}
