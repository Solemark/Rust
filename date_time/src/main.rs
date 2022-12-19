use chrono::{self, DateTime, Local};
mod test;

fn check_time() -> DateTime<Local>{
    let date: DateTime<Local> = chrono::offset::Local::now();
    date
}

fn main() {
    println!("{}", check_time());
}
