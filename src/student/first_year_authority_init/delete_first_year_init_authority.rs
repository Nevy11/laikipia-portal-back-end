use crate::{
    connecting::connection_establish::establish_connection,
    models::FirstYearInitAuthority,
    schema::first_year_init_authority::{self, id},
};
use diesel::prelude::*;
///delete
///removes a user from the database
/// returns a vector of remaining users in the database
/// if the data to be deleted is not in the table, the function returns
/// a None data type and if it is available, it returns a specific data type without panicking
/// the function returns an option of Vector of data that is deleted: the id, password, role,
/// first_name and last name of the people that are removed from the system
pub fn main_delete_user_in_first_year_init_authority(
    user_id: String,
) -> Option<Vec<FirstYearInitAuthority>> {
    let connection = &mut establish_connection();
    let result = diesel::delete(first_year_init_authority::dsl::first_year_init_authority)
        .filter(id.eq(user_id))
        .returning(FirstYearInitAuthority::as_select())
        .get_results(connection)
        .optional();

    match result {
        Ok(Some(data)) => {
            println!(
                "The data that has been removed from the system is : {:?}",
                data
            );
            Some(data)
        }
        Ok(None) => {
            println!("There is no such data in the database");
            None
        }
        Err(err) => {
            println!("An error occured while filtering the main data to be deleted: {err}");
            None
        }
    }
}
