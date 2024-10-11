use chrono::{Datelike, Local};

pub fn get_time_now() {
    let now = Local::now().date_naive();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    println!("Year: {}", year);
    println!("month: {}", month);
    println!("day: {}", day);
}
