use super::read_course_duration::get_course_in_course_duration_table;
use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::CourseDuration,
    schema::course_duration::{self, course_name},
};
use diesel::prelude::*;

// delete
pub fn delete_a_course_in_course_duration_table(course_name_in_table: String) -> CourseDuration {
    let connection = &mut establish_connection();
    let result = get_course_in_course_duration_table(course_name_in_table.clone()).unwrap();
    let returned_data = diesel::delete(course_duration::dsl::course_duration)
        .filter(course_name.eq(result.course_name.clone()))
        .returning(CourseDuration::as_returning())
        .get_result(connection)
        .expect("Failed deleting the course from the course duration table");
    returned_data
}
