use rayon::{result, vec};

use crate::masks::*;
use std::collections::HashMap;

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
        let amount_of_and_results = 2^and_mask.count_ones() as u64;

        for index in 0..amount_of_and_results {
            and_result_vector.push(get_rook_and_result(and_mask, index));
        }

        rook_and_results.push(and_result_vector);
    }
    return rook_and_results;
}

// maps the and results to the rook moves
pub fn init_rook_and_attack_map() -> HashMap<u64, u64> {
    let rook_and_results = init_rook_and_results();
    let mut rook_and_attack_map = HashMap::new();
    for i in 0..rook_and_results.len() {
        for j in 0..rook_and_results[i].len() {
        }
    }
    return rook_and_attack_map;
}


#[cfg(test)]
mod tests {
    use crate::board::display_bit_board;
    use crate::precomps_rook_logic::{get_rook_and_mask, 
                                    init_rook_and_masks,
                                    get_rook_and_result};

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
    
        for r in rook_moves {
            println!("{}", display_bit_board(r));
        }
        let rook_moves = init_rook_and_masks();

        assert_eq!(rook_moves.len(), 64);
    }

    #[test]
    fn test_handles_maximum_possible_value_for_and_mask() {
        let and_mask = get_rook_and_mask(44); // Maximum possible value for and_mask
        println!("{}", display_bit_board(and_mask));
        let result = get_rook_and_result(and_mask, 3);
        println!("{}\n{}", display_bit_board(result), result);
        assert_eq!(result, 1052672);
    }   

}