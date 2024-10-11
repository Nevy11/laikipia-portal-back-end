use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseCodes,
    schema::course_codes::{self, course_name},
};
use diesel::prelude::*;

/// Returns all the data of a particular course when returned.
/// takes in the name of the course searches the data then returns the data as a struct
/// as selectable
pub fn read_one_course_code(name_course: String) -> CourseCodes {
    let connection = &mut establish_connection();
    course_codes::dsl::course_codes
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .select(CourseCodes::as_select())
        .get_result(connection)
        .expect("Failed to read one course code from the course codes database")
}

/// the function returns all the courses as a vector from the course codes database
/// Connects with the database the loads all the data
pub fn read_all_course_codes() -> Vec<CourseCodes> {
    let connection = &mut establish_connection();
    course_codes::dsl::course_codes::load::<CourseCodes>(course_codes::table, connection)
        .expect("Failed to read all the course codes in the course_code database")
}
