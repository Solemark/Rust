mod test;

fn fizz_buzz(fizz: i8, buzz: i8, max: i8) -> String{
    let mut output: String = "".to_string();
    let mut i: i8 = 1;

    while i <= max {
        if 0 == i % fizz {
            output += "fizz";
        }
        if 0 == i % buzz {
            output += "buzz";
        }
        if !output.ends_with("z"){
            output += &i.to_string();
        }
        output += "\n";
        i += 1;
    }
    output
}

fn main() {
    println!("{}", fizz_buzz(3, 5, 20));
}
