use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseCodeAuthority,
    schema::course_code_authority::{self, user_id},
};
use diesel::prelude::*;

/// The function reads one course code authority and returns the data.
pub fn read_one_course_code_authority(user_identity: String) -> CourseCodeAuthority {
    let connection = &mut establish_connection();
    course_code_authority::dsl::course_code_authority
        .filter(user_id.eq(user_identity.to_uppercase()))
        .select(CourseCodeAuthority::as_returning())
        .get_result(connection)
        .expect("Failed to get one course code authority data")
}

/// The functions returns all the data in form of a vector of course code authority
/// returns the vector after establishing connection with the database
pub fn read_all_course_code_authority() -> Vec<CourseCodeAuthority> {
    let connection = &mut establish_connection();
    course_code_authority::dsl::course_code_authority::load(
        course_code_authority::table,
        connection,
    )
    .expect("failed to load all the data from the course_code authority table")
}

/// The function checks to see if the user is in the database.
/// if the user is in the database, It compares its result with the password stored.If it matches, Returns true, else false.
pub fn check_course_code_authority_password(user_identity: String, user_password: String) -> bool {
    let user_data = read_one_course_code_authority(user_identity.to_uppercase().clone());
    if user_data.user_password == user_password {
        true
    } else {
        false
    }
}
