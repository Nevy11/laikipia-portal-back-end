#![allow(unused_variables)]
#[allow(unused_imports)]
use chrono::NaiveDate;

// extern crate chrono;
pub fn test_chrono() {
    let utc_now = chrono::Utc::now().format("%Y %B %H");
    println!("date: {:?}", utc_now);
    let local_time = chrono::Local::now().format("%Y %B %d %H");
    println!("\n date: {:?}", local_time);
    let date1 = NaiveDate::from_isoywd_opt(2003, 2, chrono::Weekday::Sun);
    let date2 = NaiveDate::from_yo_opt(2024, 366);

    let result = date2.unwrap().format("day of the week: %A");
    println!("{}", result);
    match date1 {
        Some(day) => {
            println!("{}", day);
        }
        None => {
            println!("there is value");
        }
    }
    let date3 = NaiveDate::parse_from_str("2022/09/08", "%Y/%m/%d").unwrap();
    let today_date = chrono::Local::now().date_naive();
    println!("today_date: {:?}", today_date);
    println!("date: {:?}", date3);
    let date4 = date3 - date1.unwrap();
    println!("date difference: {:?}", date4);
    let hours = date4.num_weeks();
    println!("hours: {:?}", hours);
    let date5 = today_date - date3;
    println!("date5: {:?}", date5.num_weeks());

    let admission_year = today_date.format("%Y");
    let admission_month = today_date.format("%m");
    let admission_date = today_date.format("d");
    println!("Admission_year: {:?}", admission_year);
    println!("Admission_month: {:?}", admission_month);
    println!("Admission_date: {:?}", admission_date);
}
