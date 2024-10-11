use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseGraduationAuthority,
    schema::course_graduation_authority::{self, user_id},
};

/// The delete methods takes in the user's identity, filters it to find the actual data to delete in the database.
/// Returns a result of CourseGraduationAuthority
pub fn delete_course_graduation_authority(
    user_identity: String,
) -> Result<CourseGraduationAuthority, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(course_graduation_authority::dsl::course_graduation_authority)
        .filter(user_id.eq(user_identity.to_uppercase()))
        .returning(CourseGraduationAuthority::as_returning())
        .get_result(connection)
}
