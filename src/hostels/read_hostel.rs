use crate::{
    connecting::connection_establish::establish_connection,
    models::Hostels,
    schema::hostels::{self, hostel_name},
};
use diesel::prelude::*;

/// takes in the name of the hostel to be read and returns the a result of either the hostels or Error.
/// updates the name of the hostel to uppercase, establishes the connection to the database returning data of
/// that particular row
pub fn read_one_hostel(name_hostel: String) -> Result<Hostels, diesel::result::Error> {
    let connection = &mut establish_connection();
    hostels::dsl::hostels
        .filter(hostel_name.eq(name_hostel.to_uppercase()))
        .select(Hostels::as_returning())
        .get_result(connection)
}

/// Takes nothing in the input, returns a result of the vector to all the hostels that are stored in the hostel table.
pub fn read_all_hostels() -> Result<Vec<Hostels>, diesel::result::Error> {
    let connection = &mut establish_connection();
    hostels::dsl::hostels::load::<Hostels>(hostels::table, connection)
}
