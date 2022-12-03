#[cfg(test)]
mod tests{
    use crate::fizz_buzz;
    
    #[test]
    fn test_fizz_buzz(){
        assert_eq!(fizz_buzz(3, 5, 20), "1\n2\nfizz\n4\nbuzz\nfizz\n7\n8\nfizz\nbuzz\n11\nfizz\n13\n14\nfizzbuzz\n16\n17\nfizz\n19\nbuzz\n".to_string());
    }
}