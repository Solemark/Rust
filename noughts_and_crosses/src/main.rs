fn main(){
    let arr: [i8; 9] = [1, 0, 10,
                        10, 1, 0,
                        1, 10, 1];
    println!("{}", display_board(&arr));
    println!("{}", check_board(&arr, 0));
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

fn check_board(arr: &[i8; 9], last: usize) -> String{
    let mut output: String = String::new();
    if last == 4 {
        output.push_str(check_cells(arr[3], arr[4], arr[5]));  //x
        output.push_str(check_cells(arr[1], arr[4], arr[7]));  //y
        output.push_str(check_cells(arr[0], arr[4], arr[8]));  //z1
        output.push_str(check_cells(arr[2], arr[4], arr[6]));  //z2
    } else{
        let x1: i8;
        let x2: i8;
        let y1: i8;
        let y2: i8;
        let z: i8;
        match last{
            0 => {
                x1 = arr[1];
                x2 = arr[2];
                y1 = arr[3];
                y2 = arr[6];
                z = arr[8];
            },
            1 => {
                x1 = arr[0];
                x2 = arr[2];
                y1 = arr[4];
                y2 = arr[7];
                z = 0;
            },
            2 => {
                x1 = arr[0];
                x2 = arr[1];
                y1 = arr[5];
                y2 = arr[8];
                z = arr[6];
            },
            3 => {
                x1 = arr[4];
                x2 = arr[5];
                y1 = arr[0];
                y2 = arr[6];
                z = 0;
            },
            5 => {
                x1 = arr[3];
                x2 = arr[4];
                y1 = arr[2];
                y2 = arr[8];
                z = 0;
            },
            6 => {
                x1 = arr[7];
                x2 = arr[8];
                y1 = arr[0];
                y2 = arr[3];
                z = arr[2];
            },
            7 => {
                x1 = arr[6];
                x2 = arr[8];
                y1 = arr[1];
                y2 = arr[4];
                z = 0;
            },
            8 => {
                x1 = arr[6];
                x2 = arr[7];
                y1 = arr[2];
                y2 = arr[5];
                z = arr[0];
            },
            _ => {
                x1 = 0;
                x2 = 0;
                y1 = 0;
                y2 = 0;
                z = 0;
            }
        }
        output.push_str(check_cells(arr[last], x1, x2));    //x
        output.push_str(check_cells(arr[last], y1, y2));   //y
        output.push_str(check_cells(arr[last], arr[4], z));   //z
    }
    return output;
}

fn check_cells(a: i8, b: i8, c: i8) -> &'static str{
    match a + b + c{
        3 => return "crosses wins",
        30 => return "noughts wins",
        _=> return ""
    }
}

#[cfg(test)]
mod test{
    use crate::check_board;
    #[test]
    fn test_crosses_row(){
        assert_eq!(check_board(&[1, 1, 1,
                                0, 0, 0, 
                                0, 0, 0], 0), "crosses wins");
    }
    #[test]
    fn test_noughts_row(){
    assert_eq!(check_board(&[10, 10, 10,
                            0, 0, 0, 
                            0, 0, 0], 0), "noughts wins");
    }
    #[test]
    fn test_crosses_col(){
        assert_eq!(check_board(&[1, 0, 0,
                                1, 0, 0,
                                1, 0, 0], 0), "crosses wins");
    }
    #[test]
    fn test_noughts_col(){
        assert_eq!(check_board(&[10, 0, 0,
                                10, 0, 0,
                                10, 0, 0], 0), "noughts wins");
    }
    #[test]
    fn test_crosses_diag(){
        assert_eq!(check_board(&[1, 0, 0,
                                0, 1, 0,
                                0, 0, 1], 0), "crosses wins");
    }
    #[test]
    fn test_noughts_diag(){
        assert_eq!(check_board(&[10, 0, 0,
                                0, 10, 0,
                                0, 0, 10], 0), "noughts wins");
    }
    #[test]
    fn test_no_winner(){
        assert_eq!(check_board(&[0, 0, 0,
                                0, 0, 0,
                                0, 0, 0], 0), "");
    }
}
