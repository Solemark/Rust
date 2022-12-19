mod test;

fn remove_spaces(s: &str) -> String{
    let output: String = s.replace(" ", "");
    output
}

fn main() {
    println!("{}", remove_spaces("Hello World!"));
}
