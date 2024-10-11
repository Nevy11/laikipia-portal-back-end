use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::FeesAuthority,
    schema::fees_authority,
};

/// This function creates a user in fees authority
/// creates the data after establishing the connection with the database.
/// Adds a user in the database
pub fn create_fees_authority(data: FeesAuthority) -> FeesAuthority {
    let deleted_data = FeesAuthority {
        user_id: data.user_id.clone().to_uppercase(),
        user_password: data.user_password.clone(),
        user_first_name: data.user_first_name.clone().to_uppercase(),
        user_middle_name: data.user_middle_name.clone().to_uppercase(),
        user_last_name: data.user_last_name.clone().to_uppercase(),
        user_role: data.user_role.clone().to_uppercase(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(fees_authority::dsl::fees_authority)
        .values(&deleted_data)
        .returning(FeesAuthority::as_returning())
        .get_result(connection)
        .expect("Failed to add a fees authority data to the database")
}
