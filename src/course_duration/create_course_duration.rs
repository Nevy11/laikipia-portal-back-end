use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::CourseDuration,
    schema::course_duration::{self, course_length},
};

use diesel::prelude::*;
/// This function creates a course in course duration
/// give the name of course and length of course,
/// stores the data in course duration table
/// returns the data in course Duration format
pub fn create_course_in_course_duration_table(
    name_course: String,
    length_of_course: i32,
) -> CourseDuration {
    let connection = &mut establish_connection();
    let name_course = name_course.to_uppercase();
    let course_length = course_length;
    let data = CourseDuration {
        course_name: name_course.clone(),
        course_length: length_of_course.clone(),
    };
    diesel::insert_into(course_duration::dsl::course_duration)
        .values(&data)
        .returning(CourseDuration::as_returning())
        .get_result(connection)
        .expect("Error updating the courseDuration table")
}
