use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::TotalGraduation,
    schema::total_graduation,
};

/// This method creates the data in total_graduation table. It takes in the data in a form of TotalGraduation, inserts it into the
/// database returning a result of the TotalGraduation thus handling an error effectively
pub fn create_total_graduation(
    data: TotalGraduation,
) -> Result<TotalGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::insert_into(total_graduation::dsl::total_graduation)
        .values(data)
        .returning(TotalGraduation::as_returning())
        .get_result(connection)
}
