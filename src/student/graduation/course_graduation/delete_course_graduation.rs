use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseGraduationReturn,
    schema::course_graduation::{self, course_name, graduation_year},
};

/// This method takes in the course name and its graduation year to delete the graduation data from the database.
/// Returns a result of CourseGraduationReturn.
pub fn delete_course_graduation(
    year_of_graduation: i32,
    name_of_course: String,
) -> Result<CourseGraduationReturn, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(course_graduation::dsl::course_graduation)
        .filter(course_name.eq(name_of_course.to_uppercase()))
        .filter(graduation_year.eq(year_of_graduation))
        .returning(CourseGraduationReturn::as_returning())
        .get_result(connection)
}
