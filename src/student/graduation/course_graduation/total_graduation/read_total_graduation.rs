use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::TotalGraduation,
    schema::total_graduation::{self},
};

/// This method reads one total graduation data returning the row of the table.
/// It filters the year of graduation from the database, which is the primary key returning a result of TotalGraduation struct.
pub fn read_one_total_graduation_data(
    year_of_graduation: i32,
) -> Result<TotalGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    total_graduation::dsl::total_graduation
        .find(year_of_graduation)
        .select(TotalGraduation::as_returning())
        .get_result(connection)
}

/// the function reads all the data in the course graduation data returning a result of the vector of TotalGraduation struct.
/// uses the load method to load all the data from the database.
pub fn read_all_total_graduation_data() -> Result<Vec<TotalGraduation>, diesel::result::Error> {
    let connection = &mut establish_connection();
    total_graduation::dsl::total_graduation::load::<TotalGraduation>(
        total_graduation::table,
        connection,
    )
}
