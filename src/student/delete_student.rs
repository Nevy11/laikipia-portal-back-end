use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::Students,
    schema::students::{self, reg_no},
};

use diesel::prelude::*;

pub fn delete_student(student_id: String) -> Students {
    let connection = &mut establish_connection();

    diesel::delete(students::dsl::students)
        .filter(reg_no.eq(student_id.clone().to_uppercase()))
        .returning(Students::as_returning())
        .get_result(connection)
        .expect("Failed to remove the user from the students table")
}
