use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::Students,
    schema::students::{
        self, admission_date, admission_year, course, department, first_name, gender, gssp,
        last_name, middle_name, password, programme, reg_no, school, students_role, year_of_study,
    },
};

use diesel::prelude::*;

/// the function updates the field of the students information
/// the field is as follows
/// pub reg_no: String,
/// pub password: String,
/// pub first_name: String,
/// pub middle_name: String,
/// pub last_name: String,
/// pub year_of_study: i32,
/// pub semester: i32,
/// pub course: String,
/// pub programme: String,
/// pub department: String,
/// pub school: String,
/// pub class: String,
/// pub gssp: String,
/// pub students_role: String,
/// pub admission_date: i32,
/// pub admission_month: i32,
/// pub admission_year: i32,
/// the field is passed above is passed in the field as String,
/// Returns an Option of Students struct.
pub fn update_student(
    student_registration_no: String,
    field: String,
    new_value: String,
) -> Option<Students> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase().clone();
    let search_field = field.as_str();
    match search_field {
        "REG_NO" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(reg_no.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "PASSWORD" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(password.eq(new_value.clone()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "FIRST_NAME" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(first_name.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "MIDDLE_NAME" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(middle_name.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "LAST_NAME" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(last_name.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "YEAR_OF_STUDY" => {
            let study_year: i32 = new_value
                .parse()
                .clone()
                .expect("Failed to convert the new value to i32 year of study");
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(year_of_study.eq(study_year))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "SEMESTER" => {
            let student_semester: i32 = new_value
                .parse()
                .clone()
                .expect("Failed to convert the new value to i32 semester");
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(year_of_study.eq(student_semester))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "COURSE" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(course.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "PROGRAMME" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(programme.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "DEPARTMENT" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(department.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "SCHOOL" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(school.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "GSSP" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(gssp.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "GENDER" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(gender.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "STUDENTS_ROLE" => {
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(students_role.eq(new_value.clone().to_uppercase()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "ADMISSION_DATE" => {
            let date_of_admission: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the date to i32 date of admission");
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(admission_date.eq(date_of_admission.clone()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "ADMISSION_MONTH" => {
            let month_of_admission: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the date to i32 month of admission");
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(admission_date.eq(month_of_admission.clone()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }
        "ADMISSION_YEAR" => {
            let year_of_admission: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the date to i32 year of admission");
            let updated_data = diesel::update(students::dsl::students)
                .filter(reg_no.eq(student_registration_no.clone().to_uppercase()))
                .set(admission_year.eq(year_of_admission.clone()))
                .returning(Students::as_returning())
                .get_result(connection)
                .expect("Failed updating the registration number of the student");
            Some(updated_data)
        }

        _ => {
            println!("Please enter a valid field to update");
            None
        }
    }
}

/*
pub reg_no: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub course: String,
    pub programme: String,
    pub department: String,
    pub school: String,
    pub class: String,
    pub gssp: String,
    pub students_role: String,
    pub admission_date: i32,
    pub admission_month: i32,
    pub admission_year: i32,
*/
