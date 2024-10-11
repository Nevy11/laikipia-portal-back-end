use crate::{
    connecting::connection_establish::establish_connection, models::StudentHostel,
    schema::student_hostel,
};
use diesel::prelude::*;

/// The function creates a student in the student_hostel table in database, showing his or her registration number,
/// first, middle and last name, hostel name and hostel room_number, and gender
/// The gender helps us to allocate the student to the hostel based on their own gender
pub fn create_student_hostel(data: StudentHostel) -> Result<StudentHostel, diesel::result::Error> {
    let connection = &mut establish_connection();
    let created_student_hostel_data = StudentHostel {
        student_reg_no: data.student_reg_no.to_uppercase(),
        student_first_name: data.student_first_name.to_uppercase(),
        student_middle_name: data.student_middle_name.to_uppercase(),
        student_last_name: data.student_last_name.to_uppercase(),
        hostel_name: data.hostel_name.to_uppercase(),
        hostel_room_number: data.hostel_room_number,
        gender: data.gender.to_uppercase(),
    };
    diesel::insert_into(student_hostel::dsl::student_hostel)
        .values(created_student_hostel_data)
        .returning(StudentHostel::as_returning())
        .get_result(connection)
}
