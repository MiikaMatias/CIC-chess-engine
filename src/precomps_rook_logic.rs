use crate::masks::*;

pub fn init_rook_magics() -> Vec<u64>{
    vec![0;64]
}
pub fn init_rook_unobstructed_masks() -> Vec<u64>{
    let mut vec = Vec::<u64>::with_capacity(64);
    for i in 0..64 {
        vec[i] = get_unobstructed_took_mask(i as u64);
    }
    return vec;
}

pub fn get_unobstructed_took_mask(pos: u64) -> u64 {
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



#[cfg(test)]
mod tests {
    use crate::board::display_bit_board;
    use crate::precomps;
    use crate::precomps_rook_logic::get_unobstructed_took_mask;
    use std::sync::LazyLock;

    #[test]
    fn test_rook_move_mask() {
        let result = get_unobstructed_took_mask(44);
        println!("{}", display_bit_board(result));
        assert_eq!(result, 139090376818688);
    }

}