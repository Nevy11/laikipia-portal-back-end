use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseGraduationAuthority,
    schema::course_graduation_authority::{
        self, user_first_name, user_id, user_last_name, user_middle_name, user_password, user_role,
    },
};
use diesel::prelude::*;

/// The method takes in the user identity in which is converted to uppercase to filter the specific data to be updated from the user_id column
/// The field is converted to uppercase then str; matched against the columns to specifiy which column to update.
/// The new value, depending on it's field is used to update the database.
pub fn updated_course_graduation_authority(
    user_identity: String,
    field: String,
    new_value: String,
) -> Option<Result<CourseGraduationAuthority, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "USER_ID" => Some(
            diesel::update(course_graduation_authority::dsl::course_graduation_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_id.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationAuthority::as_returning())
                .get_result(connection),
        ),
        "USER_PASSWORD" => Some(
            diesel::update(course_graduation_authority::dsl::course_graduation_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_password.eq(new_value.clone()))
                .returning(CourseGraduationAuthority::as_returning())
                .get_result(connection),
        ),
        "USER_FIRST_NAME" => Some(
            diesel::update(course_graduation_authority::dsl::course_graduation_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_first_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationAuthority::as_returning())
                .get_result(connection),
        ),
        "USER_MIDDLE_NAME" => Some(
            diesel::update(course_graduation_authority::dsl::course_graduation_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_middle_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationAuthority::as_returning())
                .get_result(connection),
        ),
        "USER_LAST_NAME" => Some(
            diesel::update(course_graduation_authority::dsl::course_graduation_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_last_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationAuthority::as_returning())
                .get_result(connection),
        ),
        "USER_ROLE" => Some(
            diesel::update(course_graduation_authority::dsl::course_graduation_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_role.eq(new_value.to_uppercase().clone()))
                .returning(CourseGraduationAuthority::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!(
                "Enter a valid field to make the changes\nEither:\n
                    user_id,
                    user_password,
                    user_first_name,
                    user_middle_name,
                    user_last_name,
                    user_role"
            );
            None
        }
    }
}
