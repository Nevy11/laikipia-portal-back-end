use crate::connecting::connection_establish::establish_connection;
use crate::{models::Students, schema::students};

use diesel::prelude::*;
pub fn first_years_init(
    reg_no: String,
    password: String,
    first_name: String,
    last_name: String,
    middle_name: String,
    year_of_study: i32,
    semester: i32,
    programme: String,
    course: String,
    department: String,
    school: String,
    class: String,
    gssp: String,
    role: String,
    gender: String,
    admission_date: u32,
    admission_month: u32,
    admission_year: i32,
) -> Students {
    let connection = &mut establish_connection();

    let data = Students {
        reg_no: reg_no.to_uppercase(),
        password: password,
        first_name: first_name.to_uppercase(),
        middle_name: middle_name.to_uppercase(),
        last_name: last_name.to_uppercase(),
        year_of_study: year_of_study,
        semester: semester,
        programme: programme.to_uppercase(),
        course: course.to_uppercase(),
        department: department.to_uppercase(),
        school: school.to_uppercase(),
        class: class.to_uppercase(),
        gssp: gssp.to_uppercase(),
        gender: gender,
        admission_date: admission_date.try_into().unwrap(),
        admission_month: admission_month.try_into().unwrap(),
        admission_year: admission_year,
        students_role: role.to_uppercase(),
    };

    diesel::insert_into(students::dsl::students)
        .values(&data)
        .returning(Students::as_returning())
        .get_result(connection)
        .expect("Failed to add the student to the student table")
}

/*
drop database login, student_information and student init
*/
