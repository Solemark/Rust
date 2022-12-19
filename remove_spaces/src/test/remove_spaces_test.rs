#[cfg(test)]
mod tests{
    use crate::remove_spaces;

    #[test]
    fn test_hello_world_to_helloworld(){
        assert_eq!(remove_spaces("Hello World!"), "HelloWorld!".to_string());
    }
}