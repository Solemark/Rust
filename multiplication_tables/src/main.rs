fn multiplication_tables(table: i8) -> [i8; 13] {
    let mut output: [i8; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // 0 ... 12 = 13
    let mut x: i8 = 0;
    let mut i: usize = 0;

    while x <= 12 {
        output[i] = table * x;
        x += 1;
        i += 1;
    }
    output
}

fn main() {
    println!("{:?}", multiplication_tables(2));
}
