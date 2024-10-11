use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::Hostels,
    schema::hostels::{self, gender, hostel_name, no_of_rooms, students_per_room},
};

/// updates the hostel table based on the field that is provided.
/// It converts the field to uppercase, based on the columns, it matches to determine which column in hostels to update.
/// if the matching is unsuccessfull, Returns None.
/// Else, updates the specific column using the name of the hostel to find the row.
/// Returns An Option of A result of Hostels.
pub fn update_hostel(
    name_hostel: String,
    field: String,
    new_value: String,
) -> Option<Result<Hostels, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "HOSTEL_NAME" => Some(
            diesel::update(hostels::dsl::hostels)
                .filter(hostel_name.eq(name_hostel.to_uppercase()))
                .set(hostel_name.eq(new_value.to_uppercase()))
                .returning(Hostels::as_returning())
                .get_result(connection),
        ),
        "NO_OF_ROOMS" => {
            let rooms_number: i32 = new_value
                .parse()
                .expect("Failed to convert the new value to i32");
            Some(
                diesel::update(hostels::dsl::hostels)
                    .filter(hostel_name.eq(name_hostel.to_uppercase()))
                    .set(no_of_rooms.eq(rooms_number))
                    .returning(Hostels::as_returning())
                    .get_result(connection),
            )
        }
        "STUDENTS_PER_ROOM" => {
            let room_students: i32 = new_value
                .parse()
                .expect("Failed to convert the new value to i32");
            Some(
                diesel::update(hostels::dsl::hostels)
                    .filter(hostel_name.eq(name_hostel.to_uppercase()))
                    .set(students_per_room.eq(room_students))
                    .returning(Hostels::as_returning())
                    .get_result(connection),
            )
        }
        "GENDER" => Some(
            diesel::update(hostels::dsl::hostels)
                .filter(hostel_name.eq(name_hostel.to_uppercase()))
                .set(gender.eq(new_value.to_uppercase()))
                .returning(Hostels::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!(
                "Please select a valid field: Between (hostel_name, no_of_rooms, students_per_room, gender)"
            );
            None
        }
    }
}
/*
hostel_name VARCHAR PRIMARY KEY,
    no_of_rooms INTEGER NOT NULL,
    students_per_room INTEGER NOT NULL,
    gender VARCHAR NOT NULL
*/
