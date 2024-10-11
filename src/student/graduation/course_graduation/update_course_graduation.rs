use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseGraduationReturn,
    schema::course_graduation::{
        self, course_name, department, fail, first_class_students, graduation_year,
        number_students, pass, programme, second_class_lower_division_students,
        second_class_upper_division_students,
    },
};

/// This method updates the course graduation data in the databased. takes in the course name which is udpated and filtered to a specific data
/// in the table. If the field does not match, returns none.
pub fn update_course_graduation(
    name_of_course: String,
    field: String,
    new_value: String,
) -> Option<Result<CourseGraduationReturn, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "GRADUATION_YEAR" => {
            let year_of_graduation: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(graduation_year.eq(year_of_graduation))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "COURSE_NAME" => Some(
            diesel::update(course_graduation::dsl::course_graduation)
                .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                .set(course_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationReturn::as_returning())
                .get_result(connection),
        ),
        "NUMBER_STUDENTS" => {
            let value: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(number_students.eq(value))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "FIRST_CLASS_STUDENTS" => {
            let value: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(first_class_students.eq(value))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "SECOND_CLASS_UPPER_DIVISION_STUDENTS" => {
            let value: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(second_class_upper_division_students.eq(value))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "SECOND_CLASS_LOWER_DIVISION_STUDENTS" => {
            let value: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(second_class_lower_division_students.eq(value))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "PASS" => {
            let value: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(pass.eq(value))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "FAIL" => {
            let value: i32 = new_value
                .clone()
                .parse()
                .expect("Failed to convert the year of graduation into i32");
            Some(
                diesel::update(course_graduation::dsl::course_graduation)
                    .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                    .set(fail.eq(value))
                    .returning(CourseGraduationReturn::as_returning())
                    .get_result(connection),
            )
        }

        "DEPARTMENT" => Some(
            diesel::update(course_graduation::dsl::course_graduation)
                .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                .set(department.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationReturn::as_returning())
                .get_result(connection),
        ),
        "PROGRAMME" => Some(
            diesel::update(course_graduation::dsl::course_graduation)
                .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                .set(programme.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationReturn::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!(
                "
                Enter a valid field either: 
                    graduation_year,
                    course_name,
                    number_students,
                    first_class_students,
                    second_class_upper_division_students,
                    second_class_lower_division_students,
                    pass,
                    fail,
                    department,
                    programme,
            "
            );
            None
        }
    }
}
