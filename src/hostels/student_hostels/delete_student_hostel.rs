use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::StudentHostel,
    schema::student_hostel::{self, student_reg_no},
};

/// This function deletes the student data from the hostels database.
/// takes in the student's reg number, converts it to uppercase the filters it from the student_hostel table returning
/// a Result of the studentHostel struct.
pub fn delete_student_hostel(
    reg_no_student: String,
) -> Result<StudentHostel, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(student_hostel::dsl::student_hostel)
        .filter(student_reg_no.eq(reg_no_student.to_uppercase()))
        .returning(StudentHostel::as_returning())
        .get_result(connection)
}
