use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::{CourseGraduation, CourseGraduationReturn},
    schema::course_graduation::{self, course_name, graduation_year},
};

/// This method reads one course graduation data from the course_graduation table in the database
/// Takes in the name of the course and the year of graduation to return the data of that particular course in that year
///  then returns the entire data of the course_graduation table in the database
pub fn read_one_course_graduation(
    name_of_course: String,
    year_of_graduation: i32,
) -> Result<CourseGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    course_graduation::dsl::course_graduation
        .filter(course_name.eq(name_of_course.to_uppercase()))
        .filter(graduation_year.eq(year_of_graduation))
        .select(CourseGraduation::as_returning())
        .get_result(connection)
}

/// This method reads all the course graduation data from the database returning a Result of a vector of CourseGraduationReturn.
pub fn read_all_course_graduation() -> Result<Vec<CourseGraduationReturn>, diesel::result::Error> {
    let connection = &mut establish_connection();
    course_graduation::dsl::course_graduation::load::<CourseGraduationReturn>(
        course_graduation::table,
        connection,
    )
}
