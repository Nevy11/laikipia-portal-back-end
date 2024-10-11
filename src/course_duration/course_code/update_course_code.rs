use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseCodes,
    schema::course_codes::{self, course_code, course_name},
};

/// The function updates the course code
/// either the course name or code, whatever will be updated
pub fn update_course_code(
    name_course: String,
    field: String,
    new_value: String,
) -> Option<CourseCodes> {
    let connection = &mut establish_connection();
    let field_value = field.to_uppercase();
    let field_match = field_value.as_str();
    match field_match {
        "COURSE_NAME" => Some(
            diesel::update(course_codes::dsl::course_codes)
                .filter(course_name.eq(name_course.to_uppercase().clone()))
                .set(course_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodes::as_returning())
                .get_result(connection)
                .expect("Failed updating the course name"),
        ),
        "COURSE_CODE" => Some(
            diesel::update(course_codes::dsl::course_codes)
                .filter(course_name.eq(name_course.to_uppercase().clone()))
                .set(course_code.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodes::as_returning())
                .get_result(connection)
                .expect("Failed updating the course name"),
        ),
        _ => {
            println!("the field should either be Course_code or Course_name");
            None
        }
    }
}
