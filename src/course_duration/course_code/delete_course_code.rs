use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseCodes,
    schema::course_codes::{self, course_name},
};
use diesel::prelude::*;

/// The function deletes a course name and its code.
/// takes in the course name then deletes the course code.
pub fn delete_course_code(name_course: String) -> CourseCodes {
    let connection = &mut establish_connection();
    diesel::delete(course_codes::dsl::course_codes)
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .returning(CourseCodes::as_returning())
        .get_result(connection)
        .expect("Failed to delete the course code in the course code database")
}
