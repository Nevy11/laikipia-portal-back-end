use crate::{
    course_duration::{
        read_course_duration::get_course_in_course_duration_table,
        years_since_admission::years_since_admission,
    },
    student::{
        delete_student::delete_student,
        read_student::{check_student_password, read_one_student},
    },
    time::{current_day::current_day, current_month::current_month, current_year::current_year},
};

use super::update_student::update_student;

/// The function updates the student semister and current year
/// takes in the registration number, password of student
/// checks to see if the student is allowed to change his or her own year and sem
/// if is allowed, read the students data, collect the course of teh student,
/// match the students, course with that on the course duration database students course,
/// Gets the year of the course, compares it with the current year of the course, it gets by calculating the
/// difference between the current year and the admission date.
/// if its less, updaets the year and semister,
/// if its more, deletes the students data.
pub fn set_student_sem_year(
    registration_no: String,
    student_password: String,
) -> Option<crate::models::Students> {
    let is_allowed = check_student_password(
        registration_no.clone().to_uppercase(),
        student_password.clone(),
    );
    if is_allowed {
        let student_data = read_one_student(registration_no.clone().to_uppercase());
        let student_course = student_data.course;
        let unit_data = get_course_in_course_duration_table(student_course);
        match unit_data {
            Some(actual_unit_data) => {
                let length_of_course = actual_unit_data.course_length;
                // check the course length
                let day_current = current_day();
                let month_current = current_month();
                let year_current = current_year();
                let year_of_study = years_since_admission(
                    registration_no.clone().to_uppercase(),
                    day_current,
                    month_current,
                    year_current,
                );
                if length_of_course < year_of_study {
                    let update_data_year = update_student(
                        registration_no.clone().to_uppercase(),
                        String::from("year_of_study"),
                        year_of_study.to_string(),
                    );
                    println!("Updates: {:?}", update_data_year);
                    match month_current {
                        9 | 10 | 11 | 12 => {
                            let month_update_data = update_student(
                                registration_no.clone().to_uppercase(),
                                String::from("semester"),
                                1.to_string(),
                            );
                            month_update_data
                        }
                        1 | 2 | 3 | 4 => {
                            let month_update_data = update_student(
                                registration_no.clone().to_uppercase(),
                                String::from("semester"),
                                2.to_string(),
                            );
                            month_update_data
                        }
                        _ => {
                            let month_update_data = update_student(
                                registration_no.clone().to_uppercase(),
                                String::from("semester"),
                                0.to_string(),
                            );
                            month_update_data
                        }
                    }
                } else {
                    println!("You are passed from the system");
                    let deleted_student_data =
                        delete_student(registration_no.clone().to_uppercase());
                    println!("Deleted: {:?}", deleted_student_data);
                    None
                }
            }
            None => {
                println!("Failed to get the course data from the course duration table");
                None
            }
        }
    } else {
        println!("Not allowed to access the system");
        None
    }
}
