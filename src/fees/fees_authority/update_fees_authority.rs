use crate::{
    connecting::connection_establish::establish_connection,
    models::FeesAuthority,
    schema::fees_authority::{
        self, user_first_name, user_id, user_last_name, user_password, user_role,
    },
};
use diesel::prelude::*;

/// The function updates the fees authority
/// Takes in the user identity, field and new value
/// searches through the field on what to update
/// take example user_id, if the field is user id, it filters the data based on the user identity,
/// sets the new value to be the user id, converts it to uppercase then returns an option on the updated value
pub fn update_fees_authority(
    user_identity: String,
    field: String,
    new_value: String,
) -> Option<FeesAuthority> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let search_field = field.as_str();
    match search_field {
        "USER_ID" => {
            let returned_data = diesel::update(fees_authority::dsl::fees_authority)
                .filter(user_id.eq(user_identity.clone().to_uppercase()))
                .set(user_id.eq(new_value.to_uppercase().clone()))
                .returning(FeesAuthority::as_returning())
                .get_result(connection);
            match returned_data {
                Ok(ouput_data) => Some(ouput_data),
                Err(e) => {
                    println!("Failed to update");
                    println!("Error: {e:?}");
                    None
                }
            }
        }
        "USER_PASSWORD" => {
            let returned_data = diesel::update(fees_authority::dsl::fees_authority)
                .filter(user_id.eq(user_identity.clone().to_uppercase()))
                .set(user_password.eq(new_value.clone()))
                .returning(FeesAuthority::as_returning())
                .get_result(connection);
            match returned_data {
                Ok(ouput_data) => Some(ouput_data),
                Err(e) => {
                    println!("Failed to update");
                    println!("Error: {e:?}");
                    None
                }
            }
        }
        "USER_FIRST_NAME" => {
            let returned_data = diesel::update(fees_authority::dsl::fees_authority)
                .filter(user_id.eq(user_identity.clone().to_uppercase()))
                .set(user_first_name.eq(new_value.to_uppercase().clone()))
                .returning(FeesAuthority::as_returning())
                .get_result(connection);
            match returned_data {
                Ok(ouput_data) => Some(ouput_data),
                Err(e) => {
                    println!("Failed to update");
                    println!("Error: {e:?}");
                    None
                }
            }
        }
        "USER_LAST_NAME" => {
            let returned_data = diesel::update(fees_authority::dsl::fees_authority)
                .filter(user_id.eq(user_identity.clone().to_uppercase()))
                .set(user_last_name.eq(new_value.to_uppercase().clone()))
                .returning(FeesAuthority::as_returning())
                .get_result(connection);
            match returned_data {
                Ok(ouput_data) => Some(ouput_data),
                Err(e) => {
                    println!("Failed to update");
                    println!("Error: {e:?}");
                    None
                }
            }
        }
        "USER_ROLE" => {
            let returned_data = diesel::update(fees_authority::dsl::fees_authority)
                .filter(user_id.eq(user_identity.clone().to_uppercase()))
                .set(user_role.eq(new_value.to_uppercase().clone()))
                .returning(FeesAuthority::as_returning())
                .get_result(connection);
            match returned_data {
                Ok(ouput_data) => Some(ouput_data),
                Err(e) => {
                    println!("Failed to update");
                    println!("Error: {e:?}");
                    None
                }
            }
        }
        _ => {
            println!("Please pick a field that matches the update, either: User id");
            None
        }
    }
}
