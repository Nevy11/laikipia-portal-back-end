use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::StudentsGraduation,
    schema::students_graduation,
};

/// Create students graduation function links the graduation data with the backend.
/// Takes in the data that is stored and necessarry for the graduation certificate and also keep track of when the graduation occured.
/// It returns a result of the students graduation.
pub fn create_students_graduation(
    data: StudentsGraduation,
) -> Result<StudentsGraduation, diesel::result::Error> {
    let connection = &mut establish_connection();
    let student_graduation_data = StudentsGraduation {
        student_reg_no: data.student_reg_no.to_uppercase(),
        student_first_name: data.student_first_name.to_uppercase(),
        student_middle_name: data.student_middle_name.to_uppercase(),
        student_last_name: data.student_last_name.to_uppercase(),
        student_course_year: data.student_course_year,
        student_course: data.student_course.to_uppercase(),
        student_programme: data.student_programme.to_uppercase(),
        student_department: data.student_department.to_uppercase(),
        student_school: data.student_school.to_uppercase(),
        student_class: data.student_class.to_uppercase(),
        student_gssp: data.student_gssp.to_uppercase(),
        student_gender: data.student_gender.to_uppercase(),
        student_role: data.student_role,
        student_hostel: data.student_hostel,
        graduation_year: data.graduation_year,
        graduation_month: data.graduation_month,
        graduation_day: data.graduation_day,
    };
    diesel::insert_into(students_graduation::dsl::students_graduation)
        .values(student_graduation_data)
        .returning(StudentsGraduation::as_returning())
        .get_result(connection)
}
