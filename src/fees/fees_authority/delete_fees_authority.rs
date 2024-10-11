use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::FeesAuthority,
    schema::fees_authority::{self, user_id},
};

/// This function deletes a user in fees authority,
/// Takes in the user identity and returns a set of information of the deleted user.
/// Panics if the user has already been deleted
pub fn delete_fees_authority(user_identity: String) -> FeesAuthority {
    let connection = &mut establish_connection();
    diesel::delete(fees_authority::dsl::fees_authority)
        .filter(user_id.eq(user_identity.clone().to_uppercase()))
        .returning(FeesAuthority::as_returning())
        .get_result(connection)
        .expect("Failed to delete a user in fees authority table")
}
