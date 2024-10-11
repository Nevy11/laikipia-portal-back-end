use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::CourseDuration,
    schema::course_duration::{self, course_length, course_name},
};

use diesel::prelude::*;

/// update_the table
/// takes the name of the course and updates it length in years
/// returns the coure duration updated
/// updates the course_name and course_length
pub fn update_course_in_course_duration_table(
    name_of_course: String,
    new_value: String,
    field: String,
) -> Option<CourseDuration> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let search_field = field.as_str();
    match search_field {
        "COURSE_NAME" => {
            let name_data = diesel::update(course_duration::dsl::course_duration)
                .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                .set(course_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseDuration::as_returning())
                .get_result(connection)
                .expect("Failed to update the course duration table");
            Some(name_data)
        }
        "COURSE_LENGTH" => {
            let length_data: i32 = new_value
                .parse()
                .expect("failed to convert the new value to i32 ");
            let length_data_update = diesel::update(course_duration::dsl::course_duration)
                .filter(course_name.eq(name_of_course.to_uppercase().clone()))
                .set(course_length.eq(length_data))
                .returning(CourseDuration::as_returning())
                .get_result(connection)
                .expect("Failed to update the course_duration table");
            Some(length_data_update)
        }
        _ => {
            println!("Please enter either a course length or a course name");
            None
        }
    }
}
