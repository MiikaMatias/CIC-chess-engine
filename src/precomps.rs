use std::collections::HashMap;
use crate::precomps_bishop_logic::*;
use crate::precomps_knight_logic::*;
use crate::precomps_rook_logic::*;
use crate::precomps_pawn_logic::*;
use crate::config::*;
use crate::precomps_rook::*;
use crate::precomps_bishop::*;

#[derive(Clone)]
pub struct Precomps {
    knight_table: HashMap<u64, u64>,
    pawn_table: [(u64, u64); 128],

    rook_entries: [MagicEntry; 64],
    rook_table: [u64; 102400],
    rook_table_offsets: [usize; 64],

    bishop_entries: [MagicEntry; 64],
    bishop_table: [u64; 5248],
    bishop_table_offsets: [usize; 64],

    pub pawn_hash: u64,
    pub knight_hash: u64,
    pub bishop_hash: u64,
    pub rook_hash: u64,
    pub queen_hash: u64,
    pub king_hash: u64,
    pub turn_hash: u64,
}

#[derive(Clone)]
#[derive(Copy)]
pub struct MagicEntry {
    pub magic: u64,
    pub mask: u64,
    pub shift: u8,
}


impl Precomps {
    pub fn new() -> Precomps {
        if PRECOMP_ROOK {init_rook_magics(ROOK_FILE_PATH);};
        if PRECOMP_BISHOP {init_bishop_magics(BISHOP_FILE_PATH);};
        Precomps {
            knight_table: init_knight_and_masks(),

            pawn_table: init_pawn_and_masks(),

            rook_entries: ROOK_MAGIC_ENTRIES,
            rook_table: ROOK_MOVE_TABLE,
            rook_table_offsets: ROOK_MOVE_TABLE_OFFSETS,

            bishop_entries: BISHOP_MAGIC_ENTRIES,
            bishop_table: BISHOP_MOVE_TABLE,
            bishop_table_offsets: BISHOP_MOVE_TABLE_OFFSETS,

            pawn_hash: rand::random::<u64>(),
            knight_hash: rand::random::<u64>(),
            bishop_hash: rand::random::<u64>(),
            rook_hash: rand::random::<u64>(),
            queen_hash: rand::random::<u64>(),
            king_hash: rand::random::<u64>(),
            turn_hash: rand::random::<u64>(),
        }
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        return self.knight_table.get(&pos).unwrap().clone();
    }

    pub fn get_pawn_move_mask(&self, mut pos: u64, is_white: bool) -> (u64, u64) {
        if !is_white {
            pos += 64; // add offset for black
        }
        return self.pawn_table[pos as usize];
    }

    pub fn magic_index(&self, entry: &MagicEntry, blockers: u64) -> u64{
        let blockers_that_matter = blockers & entry.mask;
        let magic_mul = blockers_that_matter.wrapping_mul(entry.magic);
        return magic_mul >> (entry.shift);
    }

    pub fn get_rook_move_mask(&self, pos: u64, blockers: u64) -> u64 {
        let offset = self.rook_table_offsets[pos as usize];
        let index = self.magic_index(&self.rook_entries[pos as usize], blockers) as usize;
    
        self.rook_table[offset + index]
    }

    pub fn get_bishop_move_mask(&self, pos: u64, blockers: u64) -> u64 {
        let offset = self.bishop_table_offsets[pos as usize];
        let index = self.magic_index(&self.bishop_entries[pos as usize], blockers) as usize;
    
        self.bishop_table[offset + index]
    }

}

