use crate::{
    connecting::connection_establish::establish_connection, models::HostelsAuthority,
    schema::hostel_authority,
};
use diesel::prelude::*;

/// The method creates hostels authority users.
/// Takes in data if form of HostelsAuthority structs, sets all data except the password to uppercase
/// then inserts into hostel_authority table returning a Result of HostelsAuthority and an error
pub fn create_hostels_authority(
    data: HostelsAuthority,
) -> Result<HostelsAuthority, diesel::result::Error> {
    let connection = &mut establish_connection();
    let added_data = HostelsAuthority {
        user_id: data.user_id.to_uppercase(),
        user_password: data.user_password,
        user_first_name: data.user_first_name.to_uppercase(),
        user_middle_name: data.user_middle_name.to_uppercase(),
        user_last_name: data.user_last_name.to_uppercase(),
        user_role: data.user_role.to_uppercase(),
    };
    diesel::insert_into(hostel_authority::dsl::hostel_authority)
        .values(added_data)
        .returning(HostelsAuthority::as_returning())
        .get_result(connection)
}
