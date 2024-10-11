use crate::{
    connecting::connection_establish::establish_connection, models::Hostels, schema::hostels,
};
use diesel::prelude::*;

/// takes data in form of hostels format.
/// updates the hostel name and gender to uppercase before being inserted to the
/// hostels table by diesel.
pub fn create_hostel(data: Hostels) -> Result<Hostels, diesel::result::Error> {
    let connection = &mut establish_connection();
    let created_data = Hostels {
        hostel_name: data.hostel_name.to_uppercase(),
        no_of_rooms: data.no_of_rooms,
        students_per_room: data.students_per_room,
        gender: data.gender.to_uppercase(),
    };
    diesel::insert_into(hostels::dsl::hostels)
        .values(created_data)
        .returning(Hostels::as_returning())
        .get_result(connection)
}
