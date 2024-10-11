use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::CourseCodes,
    schema::course_codes,
};

/// This function creates a course code in the course_codes table
/// takes in the name of the course and its code the converts it into uppercase.
/// It inserts the values to the course_codes table returning A CourseCodes struct data.
/// If the operation fails, it panicks.
pub fn create_course_code(name_course: String, code_course: String) -> CourseCodes {
    let connection = &mut establish_connection();
    let data = CourseCodes {
        course_name: name_course.to_uppercase().clone(),
        course_code: code_course.to_uppercase().clone(),
    };
    diesel::insert_into(course_codes::dsl::course_codes)
        .values(&data)
        .returning(CourseCodes::as_returning())
        .get_result(connection)
        .expect("Failed to add data to the course_code")
}
