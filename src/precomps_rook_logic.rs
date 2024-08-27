use crate::masks::*;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Copy)]
pub struct MagicEntry {
    pub magic: u64,
    pub mask: u64,
    pub shift: u8,
}

pub fn init_rook_magics() -> ([MagicEntry; 64], Vec<Vec<u64>>) {
    let mut magic_entries = [MagicEntry{ magic: 0, mask: 0, shift: 0 }; 64];
    let mut move_table = vec![Vec::<u64>::new(); 64];
    let blocker_list = &init_rook_and_results();
    
    let mut pos = 0;
    for and_mask_vec in blocker_list {
        let mask = get_rook_and_mask(pos as u64);
        let index_bits = mask.count_ones(); // should this be count_ones() ?
        let shift = 64 - index_bits;
        let mut magic_entry = MagicEntry{magic: 0, 
                                        mask: mask, 
                                        shift: shift  as u8 };

        let mut tries = 0;
        loop {
            let mut still_looking = true;
            tries += 1;
            let mut table = vec![0; 1 << index_bits];
            let magic_candidate: u64 = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
            magic_entry.magic = magic_candidate;

            for and_mask in and_mask_vec {
                still_looking = true;
    
                let table_entry = &mut table[magic_index(&magic_entry, *and_mask) as usize];
                let moves = get_rook_move_from_and_mask(pos, *and_mask);
        
                if *table_entry == 0 {
                    *table_entry = moves;
                } else if *table_entry != moves {
                    still_looking = false;
                    break;
                }
            }
            if still_looking {
                println!("Found a table for {} {} at try {}", pos+1, magic_entry.magic, tries);
                magic_entries[pos as usize] = magic_entry; 
                move_table[pos as usize] = table;
                pos += 1;
                break;
            }
        }
    }
    return (magic_entries, move_table);
}

pub fn magic_index(entry: &MagicEntry, blockers: u64) -> u64{
    let blockers_that_matter = blockers & entry.mask;
    let magic_mul = blockers_that_matter.wrapping_mul(entry.magic);
    return magic_mul >> (entry.shift);
}

pub fn get_rook_and_mask(pos: u64) -> u64 {
    let mut board: u64 = 0;    
    // Generate horizontal moves to the left
    for i in (0..(pos % 8)).rev() {
        let mut square = 1 << (pos / 8 * 8 + i);
        square &= !FILE_A_MASK;
        board |= square;       
    }

    // Generate horizontal moves to the right
    for i in ((pos % 8) + 1)..8 {
        let mut square = 1 << (pos / 8 * 8 + i);
        square &= !FILE_H_MASK;
        board |= square;       
    }

    // Generate vertical moves upwards
    for i in (0..(pos / 8)).rev() {
        let mut square = 1 << (i * 8 + pos % 8);
        square &= !RANK_8_MASK;
        board |= square;       
    }
    
    // Generate vertical moves downwards
    for i in ((pos / 8) + 1)..8 {
        let mut square = 1 << (i * 8 + pos % 8);
        square &= !RANK_1_MASK;
        board |= square;       
    }
    return board;
}

pub fn init_rook_and_masks() -> [u64; 64] {
    let mut rook_masks = [0; 64];
    for i in 0..64 {
        rook_masks[i] = get_rook_and_mask(i as u64);
    }
    return rook_masks;
}


pub fn get_rook_move_from_and_mask(pos: u64, and_mask: u64) -> u64 {
    let mut board = 0;

    // Generate horizontal moves to the left
    for i in (0..(pos % 8)).rev() {
        let square = pos / 8 * 8 + i;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    // Generate horizontal moves to the right
    for i in ((pos % 8) + 1)..8 {
        let square = pos / 8 * 8 + i;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    // Generate vertical moves upwards
    for i in (0..(pos / 8)).rev() {
        let square = i * 8 + pos % 8;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    // Generate vertical moves downwards
    for i in ((pos / 8) + 1)..8 {
        let square = i * 8 + pos % 8;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;       
            break;
        }
    }

    board
}

pub fn get_rook_and_result(and_mask: u64, index: u64) -> u64 {
    let mut result = 0;
    let mut a = and_mask;
    let n = a.count_ones() as u32;
    for i in 0..n {
        let b = a.trailing_zeros();
        a &= !(1 << b);
        if index & (1 << i) == 0 {
            result |= 1 << b;
        }
    }
    result
}

pub fn init_rook_and_results() -> Vec<Vec<u64>> {
    let rook_and_masks = init_rook_and_masks();
    let mut rook_and_results: Vec<Vec<u64>> = Vec::new();

    for i in 0..rook_and_masks.len() {
        let mut and_result_vector = Vec::new();
        let and_mask = rook_and_masks[i];
        let amount_of_and_results = (2 as u64).pow(and_mask.count_ones() as u32);

        for index in 0..amount_of_and_results {
            and_result_vector.push(get_rook_and_result(and_mask, index));
        }

        rook_and_results.push(and_result_vector);
    }
    return rook_and_results;
}

#[cfg(test)]
mod tests {
    pub const ROOK_MOVE_TABLE_SIZE: usize = 102400;
    use crate::precomps_rook_logic::*;

    #[test]
    fn test_rook_move_mask() {
        let result = get_rook_and_mask(44);
        assert_eq!(result, 4624614895390720);
        let result = get_rook_and_mask(0);
        assert_eq!(result, 282578800148862);
    }

    #[test]
    fn test_rook_init() {    
        let rook_moves = init_rook_and_masks();

        assert_eq!(rook_moves.len(), 64);
    }

    #[test]
    fn test_handles_maximum_possible_value_for_and_mask() {
        let and_mask = get_rook_and_mask(44); // Maximum possible value for and_mask
        let result = get_rook_and_result(and_mask, 3);
        assert_eq!(result, 4624614894338048);
    }   

    #[test]
    fn test_init_rook_and_results() {
        let rook_and_results = init_rook_and_results();
        let mut sum = 0;
        for r in rook_and_results {
            sum += r.len();
        }
        let rook_and_results = init_rook_and_results();
        assert_eq!(rook_and_results.len(), 64);
        assert_eq!(sum, ROOK_MOVE_TABLE_SIZE)
    }


    #[test]
    fn test_init_rook_magics() {
        println!("init_rook_magics");
        let (rook_magics, rook_table) = init_rook_magics();
        assert_eq!(rook_magics.len(), 64);
        assert_eq!(rook_table[0].len(), 64);
    }

}