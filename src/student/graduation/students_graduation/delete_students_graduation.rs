use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::StudentsGraduation,
    schema::students_graduation::{self, student_reg_no},
};

/// The methods deletes a user from the students graduation table.
/// filters the registration number of the data being deleted to find the row to remove.
/// Returns a result to studentsgraduation
pub fn delete_students_graduation(
    registration_number: String,
) -> Result<StudentsGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    students_graduation::dsl::students_graduation
        .filter(student_reg_no.eq(registration_number.to_uppercase()))
        .select(StudentsGraduation::as_returning())
        .get_result(connection)
}
