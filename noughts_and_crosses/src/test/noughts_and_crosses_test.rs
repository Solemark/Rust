#[cfg(test)]
mod test{
    use crate::check_board;
    #[test]
    fn test_crosses_row(){
        assert_eq!(check_board(&[1, 1, 1,
                                0, 0, 0, 
                                0, 0, 0], 0), "crosses wins");
    }
    #[test]
    fn test_noughts_row(){
    assert_eq!(check_board(&[10, 10, 10,
                            0, 0, 0, 
                            0, 0, 0], 0), "noughts wins");
    }
    #[test]
    fn test_crosses_col(){
        assert_eq!(check_board(&[1, 0, 0,
                                1, 0, 0,
                                1, 0, 0], 0), "crosses wins");
    }
    #[test]
    fn test_noughts_col(){
        assert_eq!(check_board(&[10, 0, 0,
                                10, 0, 0,
                                10, 0, 0], 0), "noughts wins");
    }
    #[test]
    fn test_crosses_diag(){
        assert_eq!(check_board(&[1, 0, 0,
                                0, 1, 0,
                                0, 0, 1], 0), "crosses wins");
    }
    #[test]
    fn test_noughts_diag(){
        assert_eq!(check_board(&[10, 0, 0,
                                0, 10, 0,
                                0, 0, 10], 0), "noughts wins");
    }
    #[test]
    fn test_no_winner(){
        assert_eq!(check_board(&[0, 0, 0,
                                0, 0, 0,
                                0, 0, 0], 0), "");
    }
}