fn main(){
    let arr: [i8; 9] = [1, 0, 10,
                        10, 10, 0,
                        1, 1, 1];
    println!("{}", display_board(&arr));
    println!("{}", check_board(&arr));
    //TODO implement interactivity
}

fn display_board(arr: &[i8; 9]) -> String{
    let mut output: String = String::new();
    let mut s: &str = "";
    let mut i: usize = 0;
    while i <= 8{
        if arr[i] == 1 {
            s = "|X|";
        }
        if arr[i] == 10 {
            s = "|O|";
        }
        if arr[i] == 0 {
            s = "|_|";
        }
        output.push_str(s);

        s = "";
        if (i + 1) % 3 == 0 {
            s = "\n";
        } 
        output.push_str(s);
        i+=1;
    }
    return output;
}

fn check_board(arr: &[i8; 9]) -> &str{
    match arr[0] + arr[1] + arr[2]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }
    match arr[3] + arr[4] + arr[5]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }
    match arr[6] + arr[7] + arr[8]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }

    match arr[0] + arr[3] + arr[6]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }
    match arr[1] + arr[4] + arr[7]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }
    match arr[2] + arr[5] + arr[8]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }

    match arr[0] + arr[4] + arr[8]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }
    match arr[2] + arr[4] + arr[6]{
        3 => return crosses_win(),
        30 => return noughts_win(),
        _ => print!("")
    }
    return "no winner";
}

fn crosses_win() -> &'static str {
    return "crosses win";
}
fn noughts_win() -> &'static str {
    return "noughts win";
}

#[cfg(test)]
mod test{
    use crate::check_board;
    #[test]
    fn test_crosses_row(){
        assert_eq!(check_board(&[1, 1, 1,
                                0, 0, 0, 
                                0, 0, 0]), "crosses wins");
    }
    #[test]
    fn test_noughts_row(){
    assert_eq!(check_board(&[10, 10, 10,
                            0, 0, 0, 
                            0, 0, 0]), "noughts wins");
    }
    #[test]
    fn test_crosses_col(){
        assert_eq!(check_board(&[1, 0, 0,
                                1, 0, 0,
                                1, 0, 0]), "crosses wins");
    }
    #[test]
    fn test_noughts_col(){
        assert_eq!(check_board(&[10, 0, 0,
                                10, 0, 0,
                                10, 0, 0]), "noughts wins");
    }
    #[test]
    fn test_crosses_diag(){
        assert_eq!(check_board(&[1, 0, 0,
                                0, 1, 0,
                                0, 0, 1]), "crosses wins");
    }
    #[test]
    fn test_noughts_diag(){
        assert_eq!(check_board(&[10, 0, 0,
                                0, 10, 0,
                                0, 0, 10]), "noughts wins");
    }
    #[test]
    fn test_no_winner(){
        assert_eq!(check_board(&[0, 0, 0,
                                0, 0, 0,
                                0, 0, 0]), "no winner");
    }
}