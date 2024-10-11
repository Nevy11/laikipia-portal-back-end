use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::AuthorityGraduation,
    schema::authority_graduation::{self, user_id},
};

/// The function delete authority graduation takes in the user identity, sets it up to uppercase then filters its data from the
/// authority graduation database. Returns a result of AuthorityGraduation struct.
pub fn delete_authority_graduation(
    user_identity: String,
) -> Result<AuthorityGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(authority_graduation::dsl::authority_graduation)
        .filter(user_id.eq(user_identity.to_uppercase()))
        .returning(AuthorityGraduation::as_returning())
        .get_result(connection)
}
