use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::CourseGraduation,
    schema::course_graduation,
};

/// This method creates course graduation data into the database.
/// Takes in a data of type CourseGraduation, returning a Result of  CourseGraduation struct
pub fn create_course_graduation(
    data: CourseGraduation,
) -> Result<CourseGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    let input_data = CourseGraduation {
        graduation_year: data.graduation_year,
        course_name: data.course_name.to_uppercase().clone(),
        number_students: data.number_students,
        first_class_students: data.first_class_students,
        second_class_upper_division_students: data.second_class_upper_division_students,
        second_class_lower_division_students: data.second_class_lower_division_students,
        pass: data.pass,
        fail: data.fail,
        department: data.department.to_uppercase().clone(),
        programme: data.programme.to_uppercase().clone(),
    };
    diesel::insert_into(course_graduation::dsl::course_graduation)
        .values(input_data)
        .returning(CourseGraduation::as_returning())
        .get_result(connection)
}
