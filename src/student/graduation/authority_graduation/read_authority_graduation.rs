use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::AuthorityGraduation,
    schema::authority_graduation::{self, user_id},
};

/// This function reads one athority graduation data returning the result of the authority graduation struct.
/// Takes in the user_identity as a string, converts it into uppercase then filters out the data from the authority graduation table.
pub fn read_one_authority_graduation(
    user_identity: String,
) -> Result<AuthorityGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    authority_graduation::dsl::authority_graduation
        .filter(user_id.eq(user_identity.to_uppercase()))
        .select(AuthorityGraduation::as_returning())
        .get_result(connection)
}

/// The method takes nothing returning a Result of a Vector of AuthorityGraduation struct.
/// It returns all the data stored in the Authority graduation table in the database.
pub fn read_all_authority_graduation() -> Result<Vec<AuthorityGraduation>, diesel::result::Error> {
    let connection = &mut establish_connection();
    authority_graduation::dsl::authority_graduation::load::<AuthorityGraduation>(
        authority_graduation::table,
        connection,
    )
}

/// The method checks for the authority graduation passkey taking in the user id and password.
/// checks to see if the user is available in the authority curriculum table. If not found, prints out the error in the
/// console returning a false. Checks the password passed into the function and that stored in the database, if matches returns true
/// else false.
pub fn check_authority_graduation_passkey(user_identity: String, password_of_user: String) -> bool {
    let available_data = read_one_authority_graduation(user_identity);
    match available_data {
        Ok(user_data) => {
            if user_data.user_password == password_of_user {
                println!("Authority graduation user logged in successfully");
                true
            } else {
                println!("Incorrect password");
                false
            }
        }
        Err(e) => {
            println!("Error while reading the user's data: \n {e:?}");
            false
        }
    }
}
