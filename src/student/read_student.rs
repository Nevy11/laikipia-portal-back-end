use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::Students,
    schema::students::{self, reg_no},
};

use diesel::prelude::*;
pub fn read_one_student(registration_number: String) -> Students {
    let connection = &mut establish_connection();
    let result = students::dsl::students
        .filter(reg_no.eq(registration_number.clone().to_uppercase()))
        .select(Students::as_returning())
        .get_result(connection)
        .expect("Failed to fetch the students results");
    result
}

pub fn read_all_student() -> Vec<Students> {
    let connection = &mut establish_connection();
    students::dsl::students::load::<Students>(students::table, connection)
        .expect("Failed to return all the students information from students database")
}

pub fn check_student_password(registration_number: String, student_password: String) -> bool {
    let student_data = read_one_student(registration_number.clone().to_uppercase());
    if student_data.password == student_password {
        true
    } else {
        false
    }
}
