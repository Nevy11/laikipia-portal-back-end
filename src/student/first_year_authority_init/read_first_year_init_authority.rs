use crate::{
    connecting::connection_establish::establish_connection, models::FirstYearInitAuthority,
    schema::first_year_init_authority,
};
use diesel::prelude::*;
/// Read
/// returns all users that are allowed to access the database
/// in vector format
/// gets all first year init authority table
pub fn get_all_first_year_init_authority() -> Vec<FirstYearInitAuthority> {
    let connection = &mut establish_connection();
    first_year_init_authority::dsl::first_year_init_authority::load::<FirstYearInitAuthority>(
        first_year_init_authority::table,
        connection,
    )
    .unwrap_or_else(|err| panic!("Error: {err}"))
}
/// Read
/// Returns the data of one user who is allowed to access the database
/// in FirstYearStudentInitFormat
/// gets one first year init authority data
pub fn get_one_first_year_init_authority(user_id: String) -> FirstYearInitAuthority {
    let connection = &mut establish_connection();
    first_year_init_authority::dsl::first_year_init_authority
        .find(user_id.clone())
        .select(FirstYearInitAuthority::as_returning())
        .first(connection)
        .unwrap_or_else(|err| {
            panic!("the id you passed is not found in the first_year_init_authority table\nError: {err}");
        })
}

/// takes the owner id and password, checks to see
/// whether its allowed to access the table first year init,
/// in case of an error, panics.
pub fn check_first_year_init_existance(owner_identity: String, owner_password: String) -> bool {
    let data = get_one_first_year_init_authority(owner_identity);
    if owner_password == data.password {
        return true;
    } else {
        panic!("wrong password entered");
    }
}
