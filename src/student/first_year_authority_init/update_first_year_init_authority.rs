use crate::{
    connecting::connection_establish::establish_connection,
    models::FirstYearInitAuthority,
    schema::first_year_init_authority::{self, first_name, id, last_name, password, role},
};
use diesel::prelude::*;

///Update
/// Updates the password or first_name or last_name or role
/// checks the string entered by the user
/// if the user wants to update the first_name,
/// all the other values will be negleted and only the first name will be stored
/// Give it the user id whom you want to make the changes
/// the new data in form of First year INit will act as new data to be updated based
/// on the title that is passed
pub fn update_first_year_init_authority(
    user_id: String,
    field: &str,
    new_value: String,
) -> Option<Vec<FirstYearInitAuthority>> {
    let connection = &mut establish_connection();

    match field {
        "USER_ID" => {
            let result = diesel::update(
                first_year_init_authority::dsl::first_year_init_authority
                    .filter(id.eq(user_id.to_uppercase().clone())),
            )
            .set(id.eq(new_value.to_uppercase().clone()))
            .returning(FirstYearInitAuthority::as_returning())
            .get_results(connection)
            .unwrap();
            Some(result)
        }
        "USER_PASSWORD" => {
            let result = diesel::update(
                first_year_init_authority::dsl::first_year_init_authority
                    .filter(id.eq(user_id.to_uppercase().clone())),
            )
            .set(password.eq(new_value.clone()))
            .returning(FirstYearInitAuthority::as_returning())
            .get_results(connection)
            .unwrap();
            Some(result)
        }
        "USER_FIRST_NAME" => {
            let result = diesel::update(
                first_year_init_authority::dsl::first_year_init_authority
                    .filter(id.eq(user_id.to_uppercase().clone())),
            )
            .set(first_name.eq(new_value.to_uppercase().clone()))
            .returning(FirstYearInitAuthority::as_returning())
            .get_results(connection)
            .unwrap();
            Some(result)
        }
        "USER_LAST_NAME" => {
            let result = diesel::update(
                first_year_init_authority::dsl::first_year_init_authority
                    .filter(id.eq(user_id.to_uppercase().clone())),
            )
            .set(last_name.eq(new_value.to_uppercase().clone()))
            .returning(FirstYearInitAuthority::as_returning())
            .get_results(connection)
            .unwrap();
            Some(result)
        }
        "USER_ROLE" => {
            let result = diesel::update(
                first_year_init_authority::dsl::first_year_init_authority
                    .filter(id.eq(user_id.to_uppercase().clone())),
            )
            .set(role.eq(new_value.to_uppercase().clone()))
            .returning(FirstYearInitAuthority::as_returning())
            .get_results(connection)
            .unwrap();
            Some(result)
        }
        _ => {
            println!("please enter an option in the table");
            None
        }
    }
}
