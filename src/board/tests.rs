

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pawn_attack_mask_white() {
        let chessboard = Chessboard::new();
        let pos = 8; 
        let is_white = true;

        let result = chessboard.get_pawn_attack_mask(pos, is_white);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_pawn_attack_mask_black() {
        let chessboard = Chessboard::new();
        let pos = 55; 
        let is_white = false;

        let result = chessboard.get_pawn_attack_mask(pos, is_white);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_pawn_attack_mask_no_pawn() {
        let chessboard = Chessboard::new();
        let pos = 36; 
        let is_white = true;

        let result = chessboard.get_pawn_attack_mask(pos, is_white);

        assert_eq!(result, 0);
    }
}
