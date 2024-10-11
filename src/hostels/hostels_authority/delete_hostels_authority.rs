use crate::{
    connecting::connection_establish::establish_connection,
    models::HostelsAuthority,
    schema::hostel_authority::{self, user_id},
};
use diesel::prelude::*;

/// this function deletes a user_id in hostels_authority after establishing connection with the database.
/// returns a result of Hostels Authority struct and an error of data to be deleted
pub fn delete_hostels_authority(
    user_identity: String,
) -> Result<HostelsAuthority, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(hostel_authority::dsl::hostel_authority)
        .filter(user_id.eq(user_identity.to_uppercase()))
        .returning(HostelsAuthority::as_returning())
        .get_result(connection)
}
