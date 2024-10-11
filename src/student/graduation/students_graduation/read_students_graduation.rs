use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::StudentsGraduation,
    schema::students_graduation::{self, student_reg_no},
};

/// Read one Student's graduation. Takes in the student's reg no returning a Result of the student's data
/// back to the caller of the function.
pub fn read_one_students_graduation(
    registration_number: String,
) -> Result<StudentsGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    students_graduation::dsl::students_graduation
        .filter(student_reg_no.eq(registration_number.to_uppercase()))
        .select(StudentsGraduation::as_returning())
        .get_result(connection)
}

/// Returns all the data from the students graduation in form of a result of a vector of the data returned.
pub fn read_all_students_graduation() -> Result<Vec<StudentsGraduation>, diesel::result::Error> {
    let connection = &mut establish_connection();
    students_graduation::dsl::students_graduation::load::<StudentsGraduation>(
        students_graduation::table,
        connection,
    )
}
