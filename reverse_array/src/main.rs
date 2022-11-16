fn main(){
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", arr);
    println!("{:?}", reverse_array(arr));
}

fn reverse_array(mut arr: [i32; 10]) -> [i32; 10]{
    let mut x: i32;
    let mut y: i32;
    let mut i: usize = 0;
    let mut c: usize = arr.len()-1;

    while i <= c {
        x = arr[i];
        y = arr[c];
        arr[c] = x;
        arr[i] = y;
        i+=1;
        c-=1;
    }
    return arr;
}

#[cfg(test)]
mod tests{
    use crate::reverse_array;

    #[test]
    fn test_reverse_array(){
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(reverse_array(arr), [10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
    }
}