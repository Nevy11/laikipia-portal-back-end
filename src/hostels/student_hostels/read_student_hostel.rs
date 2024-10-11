use crate::{
    connecting::connection_establish::establish_connection,
    models::StudentHostel,
    schema::student_hostel::{self, student_reg_no},
};
use diesel::prelude::*;

/// The function returns a data of a particular row based on reg_no passed in to the student hostel database.
/// Returns the result of a studentHostel struct and and error to handle errors
pub fn read_one_student_hostel(user_id: String) -> Result<StudentHostel, diesel::result::Error> {
    let connection = &mut establish_connection();
    student_hostel::dsl::student_hostel
        .filter(student_reg_no.eq(user_id.to_uppercase()))
        .select(StudentHostel::as_returning())
        .get_result(connection)
}

/// The function returns all data from the student_hostel table in the database.
/// It returns the data in the form of studentHostels struct which is wrapped into a Vector the Result for
/// error handling.
pub fn read_all_student_hostel() -> Result<Vec<StudentHostel>, diesel::result::Error> {
    let connection = &mut establish_connection();
    student_hostel::dsl::student_hostel::load::<StudentHostel>(student_hostel::table, connection)
}
