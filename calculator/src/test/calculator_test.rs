#[cfg(test)]
mod tests{
    use crate::calculator;

    #[test]
    fn test_positive_addition(){
        assert_eq!(calculator(1.0, 2.0, '+'), 3.0);
    }
    #[test]
    fn test_negative_addition(){
        assert_eq!(calculator(-1.0, -2.0, '+'), -3.0);
    }
    #[test]
    fn test_mixed_addition(){
        assert_eq!(calculator(-1.0, 2.0, '+'), 1.0);
    }

    #[test]
    fn test_positive_subtraction(){
        assert_eq!(calculator(1.0, 2.0, '-'), -1.0);
    }
    #[test]
    fn test_negative_subtraction(){
        assert_eq!(calculator(-1.0, -2.0, '-'), 1.0);
    }
    #[test]
    fn test_mixed_subtraction(){
        assert_eq!(calculator(-1.0, 2.0, '-'), -3.0);
    }

    #[test]
    fn test_positive_multiplication(){
        assert_eq!(calculator(1.0, 2.0, '*'), 2.0);
    }
    #[test]
    fn test_negative_multiplication(){
        assert_eq!(calculator(-1.0, -2.0, '*'), 2.0);
    }
    #[test]
    fn test_mixed_multiplication(){
        assert_eq!(calculator(-1.0, 2.0, '*'), -2.0);
    }

    #[test]
    fn test_positive_division(){
        assert_eq!(calculator(1.0, 2.0, '/'), 0.5);
    }
    #[test]
    fn test_negative_division(){
        assert_eq!(calculator(-1.0, -2.0, '/'), 0.5);
    }
    #[test]
    fn test_mixed_division(){
        assert_eq!(calculator(-1.0, 2.0, '/'), -0.5);
    }
}