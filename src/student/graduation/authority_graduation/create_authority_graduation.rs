use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::AuthorityGraduation,
    schema::authority_graduation,
};

/// This function creates Authority graduation data and inserts it into the table in the database.
/// All the data except the password is converted to uppercase then inserted into authority_curriculum
/// Takes in data in the form of AuthorityGraduation struct returning a result of AuthorityGraduation struct.
pub fn create_authority_graduation(
    data: AuthorityGraduation,
) -> Result<AuthorityGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    let created_data = AuthorityGraduation {
        user_id: data.user_id.to_uppercase(),
        user_password: data.user_password,
        user_role: data.user_role.to_uppercase(),
        user_first_name: data.user_first_name.to_uppercase(),
        user_middle_name: data.user_middle_name.to_uppercase(),
        user_last_name: data.user_last_name.to_uppercase(),
    };
    diesel::insert_into(authority_graduation::dsl::authority_graduation)
        .values(created_data)
        .returning(AuthorityGraduation::as_returning())
        .get_result(connection)
}
