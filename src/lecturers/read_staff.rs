use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::Staff,
    schema::staff::{self, user_id},
};

use diesel::prelude::*;

/// Read one function takes in a user id returning the entire data about the user id
pub fn read_one_staff(user_identity: String) -> Result<Staff, diesel::result::Error> {
    let connection = &mut establish_connection();
    let result_returned = staff::dsl::staff::filter(staff::table, user_id.eq(user_identity))
        .select(Staff::as_returning())
        .first(connection);
    result_returned
}

/// reads all the data from the staff returning a vector of the data (staff).
pub fn read_all_staff() -> Vec<Staff> {
    let connection = &mut establish_connection();
    let result_returned = staff::dsl::staff::load::<Staff>(staff::table, connection).unwrap();
    result_returned
}

/// this function takes the user identity, if user's identity is in the database, it matches the user's password with that of
/// the database. Returns a boolean of whether or not the user has been logged on
pub fn check_for_staff_authorization(user_identity: String, user_password: String) -> bool {
    let data_result = read_one_staff(user_identity.clone().to_uppercase());
    match data_result {
        Ok(data) => {
            let password_data = data.user_password;
            if user_password == password_data {
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
