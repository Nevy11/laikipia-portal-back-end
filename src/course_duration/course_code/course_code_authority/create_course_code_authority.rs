use crate::{
    connecting::connection_establish::establish_connection, models::CourseCodeAuthority,
    schema::course_code_authority,
};
use diesel::prelude::*;

/// This function creates a user in the course code authority table
/// Takes in a data of format course code authority
/// returns the created data
pub fn create_course_code_authority(data: CourseCodeAuthority) -> CourseCodeAuthority {
    let connection = &mut establish_connection();
    let updated_data = CourseCodeAuthority {
        user_id: data.user_id.to_uppercase().clone(),
        user_password: data.user_password.clone(),
        user_first_name: data.user_first_name.clone().to_uppercase(),
        user_middle_name: data.user_middle_name.clone().to_uppercase(),
        user_last_name: data.user_last_name.clone().to_uppercase(),
        user_role: data.user_role.clone().to_uppercase(),
    };
    diesel::insert_into(course_code_authority::dsl::course_code_authority)
        .values(&updated_data)
        .returning(CourseCodeAuthority::as_returning())
        .get_result(connection)
        .expect("Failed to update the course code authority")
}
