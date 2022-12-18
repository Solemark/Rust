#[cfg(test)]
mod tests{
    use crate::check_palindrome;

    #[test]
    fn test_check_palindrome(){
        assert_eq!(check_palindrome("DAD".to_string()), true);
    }
    #[test]
    fn test_check_case_sensitivity(){
        assert_eq!(check_palindrome("Dad".to_string()), false);
    }
}