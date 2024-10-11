use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseCodeAuthority,
    schema::course_code_authority::{self, user_id},
};
use diesel::prelude::*;

/// the function takes in a user identity as a string, finds, the data then removes it from the course code authority table
pub fn delete_course_code_authority(user_identity: String) -> Option<CourseCodeAuthority> {
    let connection = &mut establish_connection();
    let result_data = diesel::delete(course_code_authority::dsl::course_code_authority)
        .filter(user_id.eq(user_identity.to_uppercase().clone()))
        .returning(CourseCodeAuthority::as_select())
        .get_result(connection);
    match result_data {
        Ok(returning_data) => Some(returning_data),
        Err(e) => {
            println!("Failed to remove the user from the course code authority");
            println!("error: {e:?}");
            None
        }
    }
}
