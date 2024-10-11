use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::AuthorityGraduation,
    schema::authority_graduation::{
        self, user_first_name, user_id, user_last_name, user_middle_name, user_password, user_role,
    },
};

/// The update function takes in the user id, field to update, and new value to set the field to be equal to returning
/// an Option of A Result of Authority Graduation struct. If the field passed in does not match any of the ones given,
/// It returns None and if there is an error in udpating any of the matched field, it returns a Result Error.
pub fn update_authority_graduation(
    user_identity: String,
    field: String,
    new_value: String,
) -> Option<Result<AuthorityGraduation, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "USER_ID" => Some(
            diesel::update(authority_graduation::dsl::authority_graduation)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_id.eq(new_value.to_uppercase().clone()))
                .returning(AuthorityGraduation::as_returning())
                .get_result(connection),
        ),
        "USER_PASSWORD" => Some(
            diesel::update(authority_graduation::dsl::authority_graduation)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_password.eq(new_value.clone()))
                .returning(AuthorityGraduation::as_returning())
                .get_result(connection),
        ),
        "USER_ROLE" => Some(
            diesel::update(authority_graduation::dsl::authority_graduation)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_role.eq(new_value.to_uppercase().clone()))
                .returning(AuthorityGraduation::as_returning())
                .get_result(connection),
        ),
        "USER_FIRST_NAME" => Some(
            diesel::update(authority_graduation::dsl::authority_graduation)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_first_name.eq(new_value.to_uppercase().clone()))
                .returning(AuthorityGraduation::as_returning())
                .get_result(connection),
        ),
        "USER_MIDDLE_NAME" => Some(
            diesel::update(authority_graduation::dsl::authority_graduation)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_middle_name.eq(new_value.to_uppercase().clone()))
                .returning(AuthorityGraduation::as_returning())
                .get_result(connection),
        ),
        "USER_LAST_NAME" => Some(
            diesel::update(authority_graduation::dsl::authority_graduation)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_last_name.eq(new_value.to_uppercase().clone()))
                .returning(AuthorityGraduation::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!(
                "Enter a valid field: either: 
                    user_id,
                    user_password,
                    user_role,
                    user_first_name,
                    user_middle_name,
                    user_last_name"
            );
            None
        }
    }
}
