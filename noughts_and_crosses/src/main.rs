use std::io::{stdin, stdout, Write};
mod test;
fn main(){
    cli();
}

fn cli(){
    let mut arr: [i8; 9] = [0, 0, 0,
                            0, 0, 0,
                            0, 0, 0];
    let game_key: &str = "0 | 1 | 2\n3 | 4 | 5\n6 | 7 | 8";
    let mut flag: bool = true;
    let mut result: String;
    
    loop{
        let mut pos: String = String::new();
        println!("{}", draw_board(&arr));
        println!("Enter the position of the next move\n{}", game_key);
        read(&mut pos);
        let pos: usize = pos.trim().parse().unwrap();
        if flag{
            arr[pos] = 1;
        } else{
            arr[pos] = 10;
        }
        flag = !flag;
        result = check_board(&arr, pos);
        if result != "" {
            println!("{}", draw_board(&arr));
            println!("{}", result);
            break;
        }
    }
}

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush!");
    stdin().read_line(input)
        .expect("Failed to read!");
}

fn draw_board(arr: &[i8; 9]) -> String{
    let mut output: String = String::new();
    let mut s: &str = "";
    let mut i: usize = 0;
    while i <= 8{
        match arr[i] {
            1 => {s = "|X|";},
            10 => {s = "|O|";},
            0 => {s = "|_|";},
            _ => println!("Unknown board type!")
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