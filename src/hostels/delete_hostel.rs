use crate::{
    connecting::connection_establish::establish_connection,
    models::Hostels,
    schema::hostels::{self, hostel_name},
};
use diesel::prelude::*;

/// Takes in the name of the hostel, converts it to uppercase, filters it in the hostel_name column.
/// If it is found, Returns A Result of Hostel, but if unsuccessfull, returns diesel,result's error.
pub fn delete_hostel(name_of_hostel: String) -> Result<Hostels, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(hostels::dsl::hostels)
        .filter(hostel_name.eq(name_of_hostel.to_uppercase()))
        .returning(Hostels::as_returning())
        .get_result(connection)
}
