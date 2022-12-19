#[cfg(test)]
mod tests{
    use chrono::{self, DateTime, Local};
    use crate::check_time;

    #[test]
    fn test_current_date_time(){    //This doesn't work because there is a space in time between answer being initialised and check_time creating a datetime variable
        let answer: DateTime<Local> = chrono::offset::Local::now();
        assert_eq!(check_time(), answer);
    }
}