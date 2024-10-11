use diesel::prelude::*;

use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::CourseDuration,
    schema::course_duration::{self, course_name},
};

/// This function takes in the name of the course as a String,
pub fn get_course_in_course_duration_table(name_course: String) -> Option<CourseDuration> {
    let connection = &mut establish_connection();
    let result = course_duration::dsl::course_duration
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .select(CourseDuration::as_select())
        .first(connection)
        .optional();
    result.expect("The course you are looking for is not in the database CourseDuration")
}

// getting all courses
pub fn get_all_courses_in_course_duration_table() -> Vec<CourseDuration> {
    let connection = &mut establish_connection();

    course_duration::dsl::course_duration::load::<CourseDuration>(
        course_duration::table,
        connection,
    )
    .expect("Error getting all the courses in the duration table")
}
