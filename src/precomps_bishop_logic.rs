use crate::masks::*;
use crate::precomps::*;
use rand;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn init_bishop_magics(output_path: &str) -> io::Result<()> {    
    let mut magic_entries = [MagicEntry{ magic: 0, mask: 0, shift: 0 }; 64];
    let mut move_table = vec![Vec::<u64>::new(); 64];
    let blocker_list = &init_bishop_and_results();
    
    let mut pos = 0;
    for and_mask_vec in blocker_list {
        let mask = get_bishop_and_mask(pos as u64);
        let index_bits = mask.count_ones();
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
                let moves = get_bishop_move_from_and_mask(pos, *and_mask);
        
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
    write_bishop_data(output_path, &magic_entries, &move_table)
}

fn write_bishop_data(
    path: &str,
    magic_entries: &[MagicEntry; 64],
    move_table: &[Vec<u64>],
) -> io::Result<()> {
    let path = Path::new(path);
    let mut file = File::create(path)?;

    writeln!(file, "use crate::precomps::*;\n")?;
    writeln!(file, "pub const BISHOP_MAGIC_ENTRIES: [MagicEntry; 64] = [")?;
    for entry in magic_entries.iter() {
        writeln!(
            file,
            "    MagicEntry {{ magic: {}, mask: {}, shift: {} }},",
            entry.magic, entry.mask, entry.shift
        )?;
    }
    writeln!(file, "];\n")?;

    let mut tail = String::new();
    let mut total_len = 0;
    let mut offsets = String::new();
    let mut current_offset = 0;

    for table in move_table {
        offsets += &format!("    {},\n", current_offset);

        for entry in table.iter() {
            tail += &format!("    {},", entry);
        }

        total_len += table.len();
        current_offset += table.len();
    }

    writeln!(file, "pub const BISHOP_MOVE_TABLE: [u64; {}] = [{}];", total_len, tail)?;

    writeln!(file, "\npub const BISHOP_MOVE_TABLE_OFFSETS: [usize; 64] = [")?;
    writeln!(file, "{}", offsets)?;
    writeln!(file, "];")?;

    Ok(())
}


pub fn magic_index(entry: &MagicEntry, blockers: u64) -> u64{
    let blockers_that_matter = blockers & entry.mask;
    let magic_mul = blockers_that_matter.wrapping_mul(entry.magic);
    return magic_mul >> (entry.shift);
}

pub fn get_bishop_and_mask(pos: u64) -> u64 {
    let mut board: u64 = 0;    
    
    // Generate moves to the top-left
    for i in 1..8 {
        let file = (pos % 8) as i64 - i;
        let rank = (pos / 8) as i64 - i;
        if file < 0 || rank < 0 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        let square_mask = (1 << square) & !FILE_A_MASK & !RANK_8_MASK;
        if square_mask == 0 {
            break;
        }
        board |= square_mask;
    }

    // Generate moves to the top-right
    for i in 1..8 {
        let file = (pos % 8) as i64 + i;
        let rank = (pos / 8) as i64 - i;
        if file >= 8 || rank < 0 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        let square_mask = (1 << square) & !FILE_H_MASK & !RANK_8_MASK;
        if square_mask == 0 {
            break;
        }
        board |= square_mask;
    }

    // Generate moves to the bottom-left
    for i in 1..8 {
        let file = (pos % 8) as i64 - i;
        let rank = (pos / 8) as i64 + i;
        if file < 0 || rank >= 8 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        let square_mask = (1 << square) & !FILE_A_MASK & !RANK_1_MASK;
        if square_mask == 0 {
            break;
        }
        board |= square_mask;
    }

    // Generate moves to the bottom-right
    for i in 1..8 {
        let file = (pos % 8) as i64 + i;
        let rank = (pos / 8) as i64 + i;
        if file >= 8 || rank >= 8 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        let square_mask = (1 << square) & !FILE_H_MASK & !RANK_1_MASK;
        if square_mask == 0 {
            break;
        }
        board |= square_mask;
    }

    board
}

pub fn init_bishop_and_masks() -> [u64; 64] {
    let mut bishop_masks = [0; 64];
    for i in 0..64 {
        bishop_masks[i] = get_bishop_and_mask(i as u64);
    }
    return bishop_masks;
}


pub fn get_bishop_move_from_and_mask(pos: u64, and_mask: u64) -> u64 {
    let mut board = 0;

    // Generate moves to the top-left
    for i in 1..8 {
        let file = (pos % 8) as i64 - i;
        let rank = (pos / 8) as i64 - i;
        if file < 0 || rank < 0 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    // Generate moves to the top-right
    for i in 1..8 {
        let file = (pos % 8) as i64 + i;
        let rank = (pos / 8) as i64 - i;
        if file >= 8 || rank < 0 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    // Generate moves to the bottom-left
    for i in 1..8 {
        let file = (pos % 8) as i64 - i;
        let rank = (pos / 8) as i64 + i;
        if file < 0 || rank >= 8 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    // Generate moves to the bottom-right
    for i in 1..8 {
        let file = (pos % 8) as i64 + i;
        let rank = (pos / 8) as i64 + i;
        if file >= 8 || rank >= 8 {
            break;
        }
        let square = (rank * 8 + file) as u64;
        if (and_mask & (1 << square)) == 0 {
            board |= 1 << square;
        } else {
            board |= 1 << square;
            break;
        }
    }

    board
}

pub fn get_bishop_and_result(and_mask: u64, index: u64) -> u64 {
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

pub fn init_bishop_and_results() -> Vec<Vec<u64>> {
    let bishop_and_masks = init_bishop_and_masks();
    let mut bishop_and_results: Vec<Vec<u64>> = Vec::new();

    for i in 0..bishop_and_masks.len() {
        let mut and_result_vector = Vec::new();
        let and_mask = bishop_and_masks[i];
        let amount_of_and_results = (2 as u64).pow(and_mask.count_ones() as u32);

        for index in 0..amount_of_and_results {
            and_result_vector.push(get_bishop_and_result(and_mask, index));
        }

        bishop_and_results.push(and_result_vector);
    }
    return bishop_and_results;
}


#[cfg(test)]
mod tests {
    pub const BISHOP_MOVE_TABLE_SIZE: usize = 5248;
    use crate::precomps_bishop_logic::*;
    use crate::graphics::display_bit_board;

    #[test]
    fn test_bishop_move_mask() {
        let result = get_bishop_and_mask(44);
        assert_eq!(result, 11259172008099840);
        let result = get_bishop_and_mask(0);
        assert_eq!(result, 18049651735527936);
    }

    #[test]
    fn test_bishop_init() {    
        let bishop_moves = init_bishop_and_masks();

        assert_eq!(bishop_moves.len(), 64);
    }

    #[test]
    fn test_handles_maximum_possible_value_for_and_mask() {
        let and_mask = get_bishop_and_mask(44); // Maximum possible value for and_mask
        let result = get_bishop_and_result(and_mask, 3);
        assert_eq!(result, 11259171940859904);
    }   

    #[test]
    fn test_init_bishop_and_results() {
        let bishop_and_results = init_bishop_and_results();
        let mut sum = 0;
        for r in bishop_and_results {
            sum += r.len();
        }
        let bishop_and_results = init_bishop_and_results();
        assert_eq!(bishop_and_results.len(), 64);
        assert_eq!(sum, BISHOP_MOVE_TABLE_SIZE)
    }


}