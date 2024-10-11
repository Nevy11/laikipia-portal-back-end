use crate::{
    connecting::connection_establish::establish_connection,
    models::HostelsAuthority,
    schema::hostel_authority::{self, user_id},
};
use diesel::prelude::*;

/// Returns one data from hostel_authority table in the data.
/// the user_identity is passed to the function to return Result of hostelsAuthority and a diesel error.
pub fn read_one_hostel_authority(
    user_identity: String,
) -> Result<HostelsAuthority, diesel::result::Error> {
    let connection = &mut establish_connection();
    hostel_authority::dsl::hostel_authority
        .filter(user_id.eq(user_identity.to_uppercase()))
        .select(HostelsAuthority::as_returning())
        .get_result(connection)
}

/// Reads and returns all information from hostel_authority table in the database in the form of Result of a vector of Hostels Authority and Error.
pub fn read_all_hostel_authority() -> Result<Vec<HostelsAuthority>, diesel::result::Error> {
    let connection = &mut establish_connection();
    hostel_authority::dsl::hostel_authority::load::<HostelsAuthority>(
        hostel_authority::table,
        connection,
    )
}

/// Checks whether the user's identity is in the database.
/// if the owner is verified, the password in hostel_authority table is compared with that that has been passed in the function.
/// if matches, returns true. Incase of anything, returns false
pub fn check_hostel_authority_authorization(owner_id: String, owner_password: String) -> bool {
    let owner_data_result = read_one_hostel_authority(owner_id.to_uppercase());
    match owner_data_result {
        Ok(owner_data) => {
            if owner_data.user_password == owner_password {
                true
            } else {
                false
            }
        }
        Err(e) => {
            println!("Error: {e:?}");
            false
        }
    }
}
