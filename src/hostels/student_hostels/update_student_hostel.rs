use crate::{
    connecting::connection_establish::establish_connection,
    models::StudentHostel,
    schema::student_hostel::{
        self, gender, hostel_name, hostel_room_number, student_first_name, student_last_name,
        student_middle_name, student_reg_no,
    },
};
use diesel::prelude::*;

/// The function updates the student_hostel table based on the field; just like other update functions
/// matches the field based on some &str uppercase values, if fails, returns None, if matches, returns an Option of result
/// of The StudentHostel.
/// the new value is converted to uppercase except in room number where it is converted to string with panick
pub fn update_student_hostel(
    reg_no_student: String,
    field: String,
    new_value: String,
) -> Option<Result<StudentHostel, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "STUDENT_REG_NO" => Some(
            diesel::update(student_hostel::dsl::student_hostel)
                .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                .set(student_reg_no.eq(new_value.to_uppercase().clone()))
                .returning(StudentHostel::as_returning())
                .get_result(connection),
        ),
        "STUDENT_FIRST_NAME" => Some(
            diesel::update(student_hostel::dsl::student_hostel)
                .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                .set(student_first_name.eq(new_value.to_uppercase().clone()))
                .returning(StudentHostel::as_returning())
                .get_result(connection),
        ),
        "STUDENT_MIDDLE_NAME" => Some(
            diesel::update(student_hostel::dsl::student_hostel)
                .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                .set(student_middle_name.eq(new_value.to_uppercase().clone()))
                .returning(StudentHostel::as_returning())
                .get_result(connection),
        ),
        "STUDENT_LAST_NAME" => Some(
            diesel::update(student_hostel::dsl::student_hostel)
                .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                .set(student_last_name.eq(new_value.to_uppercase().clone()))
                .returning(StudentHostel::as_returning())
                .get_result(connection),
        ),
        "HOSTEL_NAME" => Some(
            diesel::update(student_hostel::dsl::student_hostel)
                .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                .set(hostel_name.eq(new_value.to_uppercase().clone()))
                .returning(StudentHostel::as_returning())
                .get_result(connection),
        ),
        "HOSTEL_ROOM_NUMBER" => {
            let room_number: i32 = new_value
                .parse()
                .expect("Failed to convert the new value to i32");
            Some(
                diesel::update(student_hostel::dsl::student_hostel)
                    .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                    .set(hostel_room_number.eq(room_number))
                    .returning(StudentHostel::as_returning())
                    .get_result(connection),
            )
        }
        "GENDER" => Some(
            diesel::update(student_hostel::dsl::student_hostel)
                .filter(student_reg_no.eq(reg_no_student.to_uppercase().clone()))
                .set(gender.eq(new_value.to_uppercase().clone()))
                .returning(StudentHostel::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!(
                "Invalid field is entered. Enter either 
                student_reg_no, student_first_name, student_middle_name, student_last_name, hostel_name, hostel_room_number, gender"
            );
            None
        }
    }
}
