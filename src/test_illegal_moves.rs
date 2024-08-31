#[allow(unused_imports)]
use std::str;

#[test]
fn test_illegal_move_promo() {
    let mut cmd = assert_cmd::Command::cargo_bin("cic_bitboard").unwrap();
    let output = cmd.write_stdin("position startpos fen 1q2k3/P7/8/8/8/8/8/7K w - - 0 1\ngo\n").assert().get_output().stdout.clone();
    let output_str = str::from_utf8(&output).unwrap();
    println!("{}", output_str);
    assert_eq!(output_str.contains("bestmove a7b8q"), true);
}

#[test]
fn test_illegal_move_weird() {
    // rnbq1bnr/4k3/4p1Q1/1PNpP2p/3P3P/4P3/1pP3B1/1K1R2R1 b - - 1 27
    let mut cmd = assert_cmd::Command::cargo_bin("cic_bitboard").unwrap();
    let output = cmd.write_stdin("position startpos moves e2e4 c7c6 d2d4 d7d6 e4e5 d6d5 b1c3 d8a5 g1f3 e7e6 h2h4 f7f5 f3d2 a7a6 d2b3 a5d8 b3c5 b7b6 c5b3 b6b5 c1f4 g7g6 a2a3 h7h6 d1d2 h6h5 g2g3 a6a5 f1g2 g6g5 c3d5 c6d5 f4e3 g5h4 g3h4 b8c6 b3c5 a5a4 e1c1 b5b4 a3b4 f5f4 d2d3 f4e3 b4b5 c6b8 f2e3 a4a3 d3g6 e8e7 h1g1 a3b2 c1b1\ngo\nbr").assert().get_output().stdout.clone();
    let output_str = str::from_utf8(&output).unwrap();
    println!("{}", output_str);
    assert_ne!(output_str.contains("bestmove b2a1q"), true);
}

//rnbqkbnr/pppp1ppp/8/4p3/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 2
#[test]
fn test_move_good() {
    let mut cmd = assert_cmd::Command::cargo_bin("cic_bitboard").unwrap();
    let output = cmd.write_stdin("position startpos moves e2e3 e7e5\ngo\nbr").assert().get_output().stdout.clone();
    let output_str = str::from_utf8(&output).unwrap();
    println!("{}", output_str);
    assert_eq!(output_str.contains("bestmove b2a1q"), true);
}

//r1b1kbnr/ppp1pppp/2n5/8/1qPP4/8/PP3PPP/RNBQK1NR w KQkq - 1 6
#[test]
fn test_other_piece_en_passant() {
    let mut cmd = assert_cmd::Command::cargo_bin("cic_bitboard").unwrap();
    let output = cmd.write_stdin("position startpos moves e2e3 d7d5 e4d5 d8d5 f1b5 d5b5 d2d4 b8c6 c2c4 b5b4\nma 34\n ma 25\n gp queen \nbr \ngo\nbr").assert().get_output().stdout.clone();
    let output_str = str::from_utf8(&output).unwrap();
    println!("{} {} {}", output_str, crate::uci_wrapper::translate_move("b5b4")[0], crate::uci_wrapper::translate_move("b5b4")[1]);
    assert_ne!(output_str.contains("bestmove c4b5"), true);
}