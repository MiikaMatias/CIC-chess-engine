pub const ROOK_FILE_PATH: &'static str = "/home/miika/Projects/CIC-chess-engine/src/precomps_rook.rs";
pub const BISHOP_FILE_PATH: &'static str = "/home/miika/Projects/CIC-chess-engine/src/precomps_bishop.rs";
pub const PRECOMP_ROOK: bool = false;
pub const PRECOMP_BISHOP: bool = false;


pub const EXTENDED_CENTER:u64 = 66229406269440;
pub const PIECE_COUNT_EC: u64 = 16;
pub const DEPTH: u16 = 3;

pub const PAWN_VAL:u32 = 100;
pub const KNIGHT_VAL:u32 = 300;
pub const BISHOP_VAL:u32 = 300;
pub const ROOK_VAL:u32 = 500;
pub const QUEEN_VAL:u32 = 900;

pub const MVV_LVA_TABLE : [[f64; 8]; 8] = [
    [0.0,0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],       // None, victim None, attacker K, Q, R, B, N, P, None
    [0.0,10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 0.0], // None, victim P, attacker K, Q, R, B, N, P, None
    [0.0,20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 0.0], // None, victim N, attacker K, Q, R, B, N, P, None
    [0.0,30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 0.0], // None, victim B, attacker K, Q, R, B, N, P, None
    [0.0,40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 0.0], // None, victim R, attacker K, Q, R, B, N, P, None
    [0.0,50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 0.0], // None, victim Q, attacker K, Q, R, B, N, P, None
    [0.0,0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],       // None, victim K, attacker K, Q, R, B, N, P, None
    [0.0,0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],       // None, victim None, attacker K, Q, R, B, N, P, None
];