use std::collections::HashMap;

pub const RANK_1_MASK: u64 = 18374686479671623680;
pub const RANK_2_MASK: u64 = 71776119061217280;
pub const RANK_7_MASK: u64 = 65280;
pub const RANK_8_MASK: u64 = 255;
pub const FILE_H_MASK: u64 = 9259542123273814144;
pub const FILE_G_MASK: u64 = 4629771061636907072;
pub const FILE_B_MASK: u64 = 144680345676153346;
pub const FILE_A_MASK: u64 = 72340172838076673;

#[derive(Clone)]
pub struct Precomps {
    knight_table: HashMap<u64, u64>,
}

impl Precomps {
    pub fn new() -> Precomps {
        Precomps {
            knight_table: init_knight_table(),
        }
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        return self.knight_table.get(&pos).unwrap().clone();
    }    
}


pub fn init_knight_table() -> HashMap<u64,u64> {
    let mut knight_table: HashMap<u64, u64> = HashMap::new();
    for pos in 0..64 {
        knight_table.insert(pos, precompute_knight_move(pos));
    }

    return knight_table;
}

pub fn precompute_knight_move(pos: u64) -> u64 {
    let in_a_file: bool = ((1u64 << pos) | FILE_A_MASK) == FILE_A_MASK;
    let in_b_file = ((1u64 << pos) | FILE_B_MASK) == FILE_B_MASK;
    let in_g_file = ((1u64 << pos) | FILE_G_MASK) == FILE_G_MASK;
    let in_h_file = ((1u64 << pos) | FILE_H_MASK) == FILE_H_MASK;
    let in_1_rank = ((1u64 << pos) | RANK_1_MASK) == RANK_1_MASK;
    let in_2_rank = ((1u64 << pos) | RANK_2_MASK) == RANK_2_MASK;
    let in_7_rank = ((1u64 << pos) | RANK_7_MASK) == RANK_7_MASK;
    let in_8_rank = ((1u64 << pos) | RANK_8_MASK) == RANK_8_MASK;

    if in_a_file {
        //done
        if in_8_rank {
            (1u64 << (pos+10)) | (1u64 << (pos+17))
        } else if in_7_rank {
            (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6))
        } else if in_2_rank {
            (1u64 << (pos-15)) | (1u64 << (pos+10)) | (1u64 << (pos-6))
        } else if in_1_rank {
            (1u64 << (pos-15)) | (1u64 << (pos-6))
        } else {
            (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6)) | (1u64 << (pos-15))
        }
    } else if in_b_file {
        if in_8_rank {
            (1u64 << (pos+15)) |  (1u64 << (pos+10)) |  (1u64 << (pos+17))
        } else if in_7_rank {
            (1u64 << (pos+17)) | (1u64 << (pos+10)) | (1u64 << (pos-6)) | (1u64 << (pos-6))
        } else if in_2_rank {
            (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6)) | (1u64 << (pos+10))
        } else if in_1_rank {
            (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6))
        } else {
            (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
        }
    } else if in_g_file {
        if in_8_rank {
            (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6))
        } else if in_7_rank {
            (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) 
        } else if in_2_rank {
            (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos+6)) | (1u64 << (pos-10))
        } else if in_1_rank {
            (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-10))
        } else {
            (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
        }
    } else if in_h_file {
        if in_8_rank {
            (1u64 << (pos+15)) | (1u64 << (pos+6))
        } else if in_7_rank {
            (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos-10)) 
        } else if in_2_rank {
            (1u64 << (pos-17)) | (1u64 << (pos+6)) | (1u64 << (pos-10))
        } else if in_1_rank {
            (1u64 << (pos-10)) | (1u64 << (pos-17))
        } else {
            (1u64 << (pos+6)) | (1u64 << (pos+15)) | (1u64 << (pos-10)) | (1u64 << (pos-17))
        }
    } else {
        //done
        if in_8_rank {
            (1u64 << (pos+10)) | (1u64 << (pos+17)) | (1u64 << (pos+6)) | (1u64 << (pos+15))
        } else if in_7_rank {
            (1u64 << (pos+10)) | (1u64 << (pos+17)) | (1u64 << (pos+6)) | (1u64 << (pos+15)) | (1u64 << (pos-6))  | (1u64 << (pos-10))
        } else if in_2_rank {
            (1u64 << (pos-10)) | (1u64 << (pos-17)) | (1u64 << (pos-6)) | (1u64 << (pos-15))  | (1u64 << (pos+6))  | (1u64 << (pos+10))
        } else if in_1_rank {
            (1u64 << (pos-10)) | (1u64 << (pos-17)) | (1u64 << (pos-6)) | (1u64 << (pos-15))
        } else {
            (1u64 << (pos+17)) | (1u64 << (pos+15)) | (1u64 << (pos+6)) | (1u64 << (pos+10)) | (1u64 << (pos-17)) | (1u64 << (pos-15)) | (1u64 << (pos-6)) | (1u64 << (pos-10)) 
        }
    }
}


