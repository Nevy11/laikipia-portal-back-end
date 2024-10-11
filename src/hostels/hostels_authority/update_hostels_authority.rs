use crate::{
    connecting::connection_establish::establish_connection,
    models::HostelsAuthority,
    schema::hostel_authority::{
        self, user_first_name, user_id, user_last_name, user_password, user_role,
    },
};
use diesel::prelude::*;

/// This method updates the hostel_authority data in the database
/// takes in the user identity, field to update, and new value in a string format.
/// if the field does not match the one provided, it returns None.
/// else returns an option of Result of HostelsAuthority
pub fn update_hostels_authority(
    user_identity: String,
    field: String,
    new_value: String,
) -> Option<Result<HostelsAuthority, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field_string = field.to_uppercase();
    let field_str = field_string.as_str();
    match field_str {
        "USER_ID" => Some(
            diesel::update(hostel_authority::dsl::hostel_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_id.eq(new_value.to_uppercase().clone()))
                .returning(HostelsAuthority::as_select())
                .get_result(connection),
        ),
        "USER_PASSWORD" => Some(
            diesel::update(hostel_authority::dsl::hostel_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_password.eq(new_value.clone()))
                .returning(HostelsAuthority::as_select())
                .get_result(connection),
        ),
        "USER_FIRST_NAME" => Some(
            diesel::update(hostel_authority::dsl::hostel_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_first_name.eq(new_value.to_uppercase().clone()))
                .returning(HostelsAuthority::as_select())
                .get_result(connection),
        ),
        "USER_LAST_NAME" => Some(
            diesel::update(hostel_authority::dsl::hostel_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_last_name.eq(new_value.to_uppercase().clone()))
                .returning(HostelsAuthority::as_select())
                .get_result(connection),
        ),
        "USER_ROLE" => Some(
            diesel::update(hostel_authority::dsl::hostel_authority)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_role.eq(new_value.to_uppercase().clone()))
                .returning(HostelsAuthority::as_select())
                .get_result(connection),
        ),
        _ => {
            println!("Please enter a valid field to update data in hostels authority");
            println!("Either user_id, user_password, user_first_name, user_last_name or user_role");
            None
        }
    }
}
