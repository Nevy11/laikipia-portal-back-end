use crate::{
    connecting::connection_establish::establish_connection, models::FirstYearInitAuthority,
    schema::first_year_init_authority,
};
use diesel::prelude::*;

/// Create a crud operation here
/// creates  a new user in the init_authority table
pub fn create_first_year_init_authority(
    user_id: String,
    user_password: String,
    user_first_name: String,
    user_middle_name: String,
    user_last_name: String,
    user_role: String,
) -> Vec<FirstYearInitAuthority> {
    let connection = &mut establish_connection();
    let data = FirstYearInitAuthority {
        id: user_id,
        password: user_password,
        first_name: user_first_name,
        middle_name: user_middle_name,
        last_name: user_last_name,
        role: user_role,
    };
    diesel::insert_into(first_year_init_authority::dsl::first_year_init_authority)
        .values(&data)
        .returning(FirstYearInitAuthority::as_returning())
        .get_results(connection)
        .unwrap_or_else(|err| {
            panic!("Error inserting into first_year_init_authority table\nError:{err}")
        })
    // format!("user has been added successfully in the first_year_course_init_authority table")
}
