use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseCodeAuthority,
    schema::course_code_authority::{
        self, user_first_name, user_id, user_last_name, user_middle_name, user_password, user_role,
    },
};
use diesel::prelude::*;

/// The function takes user_identity, field and the new_value.
/// matches the field to choose which column of a particular user to upgrade. Once the column has been found, the user is located
/// and his or her data is updated.The function returns an Option of the CourseCodeAuthority.
pub fn update_course_code_authority(
    user_identity: String,
    field: String,
    new_value: String,
) -> Option<CourseCodeAuthority> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "USER_ID" => Some(
            diesel::update(course_code_authority::dsl::course_code_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_id.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodeAuthority::as_returning())
                .get_result(connection)
                .expect("Failed to update the user id in the course code authority table"),
        ),
        "USER_PASSWORD" => Some(
            diesel::update(course_code_authority::dsl::course_code_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_password.eq(new_value.clone()))
                .returning(CourseCodeAuthority::as_returning())
                .get_result(connection)
                .expect("Failed to update the user id in the course code authority table"),
        ),
        "USER_FIRST_NAME" => Some(
            diesel::update(course_code_authority::dsl::course_code_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_first_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodeAuthority::as_returning())
                .get_result(connection)
                .expect("Failed to update the user id in the course code authority table"),
        ),
        "USER_MIDDLE_NAME" => Some(
            diesel::update(course_code_authority::dsl::course_code_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_middle_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodeAuthority::as_returning())
                .get_result(connection)
                .expect("Failed to update the user id in the course code authority table"),
        ),
        "USER_LAST_NAME" => Some(
            diesel::update(course_code_authority::dsl::course_code_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_last_name.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodeAuthority::as_returning())
                .get_result(connection)
                .expect("Failed to update the user id in the course code authority table"),
        ),
        "USER_ROLE" => Some(
            diesel::update(course_code_authority::dsl::course_code_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_role.eq(new_value.to_uppercase().clone()))
                .returning(CourseCodeAuthority::as_returning())
                .get_result(connection)
                .expect("Failed to update the user id in the course code authority table"),
        ),
        _ => {
            println!("Enter an option either User id, user password, user_first_name, user_middle_name, user_last_name or user role to update");
            None
        }
    }
}
