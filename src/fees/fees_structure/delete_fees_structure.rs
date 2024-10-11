use crate::{
    connecting::connection_establish::establish_connection,
    models::FeeStructure,
    schema::fees_structure::{self, course_name, expenditure_name, semester, year},
};
use diesel::prelude::*;

/// takes course name, year, semester, expenditure and returns all the information abouth the data that has been deleted
/// if deletion it fails, it prints out "Failed to delete the expenditure from the fees_structure table" in the screen
pub fn delete_expenditure_fee_structure(
    name_course: String,
    year_course: i32,
    sem_course: i32,
    name_expenditure: String,
) -> FeeStructure {
    let connection = &mut establish_connection();
    diesel::delete(fees_structure::dsl::fees_structure)
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .filter(year.eq(year_course))
        .filter(semester.eq(sem_course))
        .filter(expenditure_name.eq(name_expenditure.to_uppercase().clone()))
        .returning(FeeStructure::as_returning())
        .get_result(connection)
        .expect("Failed to delete the expenditure from the fees_structure table")
}
