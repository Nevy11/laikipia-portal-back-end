#![allow(unused_variables)]

use chrono::NaiveDate;

pub fn get_time_sign_up() {
    let sign_up_date = chrono::Local::now();
    println!("date of admission: {:?}", sign_up_date);
    let native_sign_up_date = NaiveDate::from_ymd_opt(2022, 9, 8).unwrap();

    let today_date = NaiveDate::from_ymd_opt(2025, 9, 8).unwrap();
    let date_difference = today_date.years_since(native_sign_up_date).unwrap();
    println!("date difference: {}", date_difference);
}
