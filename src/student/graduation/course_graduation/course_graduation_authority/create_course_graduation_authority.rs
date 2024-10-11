use crate::{
    connecting::connection_establish::establish_connection,
    models::{CourseGraduationAuthority, CourseGraduationAuthorityReturn},
    schema::course_graduation_authority,
};
use diesel::prelude::*;

/// This function creates a course graduation authority, i.e., the people that are allowed to update,
/// create and delete the graduation authority database. It converts all fields passed into it, into uppercase except the
/// user_password
pub fn create_course_graduation_authority(
    data: CourseGraduationAuthority,
) -> Result<CourseGraduationAuthorityReturn, diesel::result::Error> {
    let connection = &mut establish_connection();
    let input_data = CourseGraduationAuthority {
        user_id: data.user_id.to_uppercase(),
        user_password: data.user_password,
        user_first_name: data.user_first_name.to_uppercase(),
        user_middle_name: data.user_middle_name.to_uppercase(),
        user_last_name: data.user_last_name.to_uppercase(),
        user_role: data.user_role.to_uppercase(),
    };
    diesel::insert_into(course_graduation_authority::dsl::course_graduation_authority)
        .values(input_data)
        .returning(CourseGraduationAuthorityReturn::as_returning())
        .get_result(connection)
}
