use std::io::{stdin, stdout, Write};

mod test;

fn main() {
    cli();
}

fn cli(){
    loop {
        let mut num1: String = String::new();
        let mut num2: String = String::new();
        let mut operator: String = String::new();

        print!("Enter first number:  ");
        read(&mut num1);

        print!("Enter second number:  ");
        read(&mut num2);

        print!("Enter operation [+-*/]:  ");
        read(&mut operator);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators: String = String:: from("+-*/");

        if !operators.contains(operator){
            println!("unknown operator!");
            continue;
        }
        calculator(num1, num2, operator);
    }
}

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush!");
    stdin().read_line(input)
        .expect("Failed to read!");
}

fn calculator(num1: f32, num2: f32, operator: char) -> f32 {
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("operator error!")
    };
    println!("{} {} {} = {}", num1, operator, num2, result);
    return result;
}