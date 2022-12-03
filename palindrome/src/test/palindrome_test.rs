#[cfg(test)]
mod tests{
    use crate::check_palindrome;

    #[test]
    fn test_check_palindrome(){
        let input: String = String::from("DAD");
        assert_eq!(check_palindrome(&input), true);
    }
    #[test]
    fn test_check_case_sensitivity(){
        let input: String = String::from("Dad");
        assert_eq!(check_palindrome(&input), false);
    }
}