use std::collections::HashMap;
use crate::precomps_bishop_logic::init_bishop_magics;
use crate::precomps_bishop_logic::init_bishop_and_masks;
use crate::precomps_knight_logic::init_knight_and_masks;
use crate::precomps_rook_logic::init_rook_magics;
use crate::precomps_rook_logic::init_rook_and_masks;

pub const ROOK_MOVE_TABLE_SIZE: usize = 102400;
pub const BISHOP_MOVE_TABLE_SIZE: usize = 5248;


#[derive(Clone)]
pub struct Precomps {
    knight_table: HashMap<u64, u64>,

    rook_magics: Vec<u64>,
    rook_and_masks: Vec<u64>,
    rook_and_results: Vec<u64>,

    bishop_magics: Vec<u64>,
    bishop_and_masks: Vec<u64>,
    bishop_and_results: Vec<u64>,
}

impl Precomps {
    pub fn new() -> Precomps {
        Precomps {
            knight_table: init_knight_and_masks(),
            
            rook_magics: init_rook_magics(),
            rook_and_masks: init_rook_and_masks(),
            rook_and_results: init_rook_and_masks(),

            bishop_magics: init_bishop_magics(),
            bishop_and_masks: init_bishop_and_masks(),
            bishop_and_results: init_rook_and_masks(),
        }
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        return self.knight_table.get(&pos).unwrap().clone();
    }

    pub fn get_rook_move_mask(&self, square: u64, blockers: u64) -> u64 {
        let and_result = blockers & self.rook_and_masks[square as usize];

        return 0; // moves needs to be pre-generated
    }

    pub fn get_bishop_move_mask(&self, square: u64, blockers: u64) -> u64 {
        let and_result = blockers & self.bishop_and_masks[square as usize];

        return 0 // moves needs to be pre-generated
    }
}
