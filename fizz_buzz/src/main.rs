mod test;

fn main() {
    println!("{}", fizz_buzz(3, 5, 20));
}

fn fizz_buzz(fizz: i32, buzz: i32, max: i32) -> String{
    let mut output: String = String::new();
    let mut s: String = String::new();
    let mut a: String;
    let mut i: i32 = 1;
    while &i<= &max{
        s.drain(..);
        if i % fizz == 0{
            s.push_str("fizz");
        }
        if i % buzz == 0{
            s.push_str("buzz");
        }
        if s == ""{
            a = i.to_string();
            s.push_str(&a);
        }
        s.push_str("\n");
        output.push_str(&s);
        i+=1;
    }
    return output;
}