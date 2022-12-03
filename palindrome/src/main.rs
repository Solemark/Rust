mod test;

fn main() {
    let input: String = String::from("DAD");
    println!("{}", check_palindrome(&input));
}

fn check_palindrome(input: &String) -> bool{
    if input == &reverse_string(&input) {
        return true;
    } else{
        return false;
    }
}

fn reverse_string(s: &str) -> String{
    return s.chars().rev().collect();
}