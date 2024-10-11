use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::TotalGraduation,
    schema::total_graduation::{self, graduation_year},
};

/// The methods deletes a graduation year in case of an error. Takes in the year of graduation, filters it in the table
///  to obtain the row of the table the returns a result of TotalGraduation struct of the deleted data
pub fn delete_total_graduation(
    year_of_graduation: i32,
) -> Result<TotalGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(total_graduation::dsl::total_graduation)
        .filter(graduation_year.eq(year_of_graduation))
        .returning(TotalGraduation::as_returning())
        .get_result(connection)
}
