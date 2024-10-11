#![allow(unused_variables, unused_imports)]
use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::StudentsGraduation,
    schema::students_graduation::{
        self, graduation_day, graduation_month, graduation_year, student_class, student_course,
        student_course_year, student_department, student_gender, student_gssp, student_hostel,
        student_last_name, student_middle_name, student_programme, student_reg_no, student_role,
        student_school,
    },
};
pub fn update_student_hostel_role_graduation(
    registration_number: String,
    field: String,
    new_vector: Option<Vec<Option<String>>>,
) -> Option<Result<StudentsGraduation, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "STUDENT_HOSTEL" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_number.to_uppercase()))
                .set(student_hostel.eq(new_vector))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_ROLE" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_number.to_uppercase()))
                .set(student_role.eq(new_vector))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!("Please enter field to be either: student_hostel, or student_role");
            None
        }
    }
}

/// The function updates all the students graduation table exept the student hostel and role of the student.
/// matches the field if the field is available, it result an option or result of Students graduation.
/// The field is converted to uppercase then into str.
/// The new value in integers such as graduation dates, new value is converted to i32 value and if it fails
/// it returns a message telling us it failed to update the data.
pub fn update_students_graduation(
    registration_no: String,
    field: String,
    new_value: String,
) -> Option<Result<StudentsGraduation, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "STUDENT_FIRST_NAME" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_reg_no.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_MIDDLE_NAME" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_middle_name.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_LAST_NAME" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_last_name.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_COURSE" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_course.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_PROGRAMME" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_programme.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_DEPARTMENT" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_department.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_SCHOOL" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_school.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_CLASS" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_class.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_GSSP" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_gssp.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_DEPARTMENTP" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_gender.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_GENDER" => Some(
            diesel::update(students_graduation::dsl::students_graduation)
                .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                .set(student_gssp.eq(new_value.to_uppercase()))
                .returning(StudentsGraduation::as_returning())
                .get_result(connection),
        ),
        "STUDENT_COURSE_YEAR" => {
            let course_year: i32 = new_value
                .parse()
                .expect("failed to convert the new value to i32");
            Some(
                diesel::update(students_graduation::dsl::students_graduation)
                    .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                    .set(student_course_year.eq(course_year))
                    .returning(StudentsGraduation::as_returning())
                    .get_result(connection),
            )
        }
        "GRADUATION_YEAR" => {
            let value: i32 = new_value
                .parse()
                .expect("failed to convert the new value to i32");
            Some(
                diesel::update(students_graduation::dsl::students_graduation)
                    .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                    .set(graduation_year.eq(value))
                    .returning(StudentsGraduation::as_returning())
                    .get_result(connection),
            )
        }
        "GRADUATION_MONTH" => {
            let value: i32 = new_value
                .parse()
                .expect("failed to convert the new value to i32");
            Some(
                diesel::update(students_graduation::dsl::students_graduation)
                    .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                    .set(graduation_month.eq(value))
                    .returning(StudentsGraduation::as_returning())
                    .get_result(connection),
            )
        }
        "GRADUATION_DAY" => {
            let value: i32 = new_value
                .parse()
                .expect("failed to convert the new value to i32");
            Some(
                diesel::update(students_graduation::dsl::students_graduation)
                    .filter(student_reg_no.eq(registration_no.to_uppercase().clone()))
                    .set(graduation_day.eq(value))
                    .returning(StudentsGraduation::as_returning())
                    .get_result(connection),
            )
        }
        // remaining with vectors
        _ => {
            println!(
                "please enter a valid field: either
                student_reg_no
student_first_name,
student_middle_name,
student_last_name,
student_course_year, integer
student_course,
student_programme,
student_department,
student_school,
student_class,
student_gssp,
student_gender,
student_role, // Expect role and hostels
student_hostel,
graduation_year,
graduation_month,
graduation_day"
            );
            None
        }
    }
}
