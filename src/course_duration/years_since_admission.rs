use chrono::NaiveDate;

use crate::student::read_student::read_one_student;

/// method takes in reg_no, password, current day, month and year
/// converts it into naiveDate
/// converts admission_date into naiveDate,
/// returns the years since admission giving the year of study
/// does this by get_data() method that takes reg_no and password
/// returns the data of that particular student based on reg_no and password
/// a method returns some(data) or None of a value of the reg_no or Email
pub fn years_since_admission(reg_no: String, day: u32, month: u32, year: i32) -> i32 {
    let current_date = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .unwrap_or_else(|| panic!("failed converting the current date to naiveDate"));
    let data = read_one_student(reg_no.clone().to_uppercase());
    let admisn_date = data.admission_date;
    let admisn_mnth = data.admission_month;
    let admisn_year = data.admission_year;

    let admission_date = NaiveDate::from_ymd_opt(
        admisn_year,
        admisn_mnth.try_into().unwrap(),
        admisn_date.try_into().unwrap(),
    )
    .unwrap_or_else(|| panic!("failed converting the admission date to naiveDate"));

    let year_difference = current_date.years_since(admission_date).unwrap();
    year_difference.try_into().unwrap()
}
