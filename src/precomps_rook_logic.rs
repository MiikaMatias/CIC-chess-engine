use rayon::{result, vec};

use crate::masks::*;
use std::collections::HashMap;

struct RookMagic {
    magic: u64,
    mask: u64,
    shift: u8,
    offset: u32,
}

pub fn init_rook_magics() -> Vec<u64>{
    vec![0;64]
}
pub fn init_rook_and_masks() -> Vec<u64>{
    let mut vec = Vec::<u64>::with_capacity(64);
    for i in 0..64 {
        vec.push(get_rook_and_mask(i as u64));
    }
    return vec;
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

// maps the and results to the rook moves
pub fn init_rook_and_attack_map() -> HashMap<u64, Vec<u64>> {
    let rook_and_results = init_rook_and_results();
    let mut rook_and_attack_map = HashMap::<u64, Vec<u64>>::new();

    for pos in 0..rook_and_results.len() {
        for index in 0..rook_and_results[pos].len() {
            let move_from_and = get_rook_move_from_and_mask(pos as u64, 
                                                            rook_and_results[pos][index]);
            if rook_and_attack_map.contains_key(&move_from_and) {
                rook_and_attack_map.get_mut(&move_from_and).unwrap().push(rook_and_results[pos][index]);
            } else {
                let mut new_vec = Vec::<u64>::new();
                new_vec.push(rook_and_results[pos][index]);
                rook_and_attack_map.insert(move_from_and, new_vec.clone());
            }
        } 
    }
    return rook_and_attack_map;
}


#[cfg(test)]
mod tests {
    pub const ROOK_MOVE_TABLE_SIZE: usize = 102400;
    use crate::board::display_bit_board;
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
    fn test_init_rook_and_attack_map() {
        let rook_and_attack_map = init_rook_and_attack_map();
        assert_eq!(rook_and_attack_map.len(), 4900);
    }

}