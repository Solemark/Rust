mod test;

fn check_palindrome(input: String) -> bool{
    let mut i: usize = 0;
    let mut c: usize = ((&input.chars().count() - 1)).try_into().unwrap();
    while i <= c {
        if input.chars().nth(i).unwrap() != input.chars().nth(c).unwrap() {
            return false;
        }
        i += 1;
        c -= 1;
    }
    return true;
}

fn main() {
    println!("{}", check_palindrome("ABCDCBA".to_string()));
    println!("{}", check_palindrome("ABCDcba".to_string()));
}