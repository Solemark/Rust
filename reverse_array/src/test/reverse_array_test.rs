#[cfg(test)]
mod tests{
    use crate::reverse_array;

    #[test]
    fn test_reverse_array(){
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(reverse_array(arr), [10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
    }
}