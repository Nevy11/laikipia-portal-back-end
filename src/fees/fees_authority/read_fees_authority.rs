use crate::{
    connecting::connection_establish::establish_connection,
    models::FeesAuthority,
    schema::fees_authority::{self, user_id},
};
use diesel::prelude::*;

/// The function takes in the user identity as a string, filters the data in the table then returns a FeesAuthority struct
/// data: panics if the data is not available in the server
pub fn read_one_fees_authority(user_identity: String) -> FeesAuthority {
    let connection = &mut establish_connection();
    fees_authority::dsl::fees_authority
        .filter(user_id.eq(user_identity.clone().to_uppercase()))
        .select(FeesAuthority::as_returning())
        .get_result(connection)
        .expect("Failed to read one user in Fees authority table")
}

/// Returns all the fees in the fees authority, in a form of vector of FeesAuthority
/// panics if it fails
pub fn read_all_fees_authority() -> Vec<FeesAuthority> {
    let connection = &mut establish_connection();
    fees_authority::dsl::fees_authority::load::<FeesAuthority>(fees_authority::table, connection)
        .expect("Failed to load all users in fees authority table")
}

/// Checks whether the username or password is incorrect
/// panics if the username is incorrect,
/// returns true if both the username and password are correct
pub fn check_password_fees_authority(user_identity: String, password_user: String) -> bool {
    let users_data = read_one_fees_authority(user_identity.clone().to_uppercase());
    if users_data.user_password.clone() == password_user.clone() {
        true
    } else {
        false
    }
}
