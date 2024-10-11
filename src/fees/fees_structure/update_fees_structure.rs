use crate::{
    connecting::connection_establish::establish_connection,
    models::FeeStructure,
    schema::fees_structure::{
        self, course_name, expenditure_cost, expenditure_name, semester, year,
    },
};
use diesel::prelude::*;

/// The method updates the expenditure name and expenditure cost.
/// if field does not match anything, It returns None.
pub fn update_ependiture_fees_structure(
    name_course: String,
    course_year: i32,
    course_sem: i32,
    name_expenditure: String,
    field: String,
    new_value: String,
) -> Option<FeeStructure> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let searched_field = field.as_str();
    match searched_field {
        "EXPENDITURE_NAME" => Some(
            diesel::update(fees_structure::dsl::fees_structure)
                .filter(course_name.eq(name_course.to_uppercase().clone()))
                .filter(year.eq(course_year))
                .filter(semester.eq(course_sem))
                .filter(expenditure_name.eq(name_expenditure.to_uppercase().clone()))
                .set(expenditure_name.eq(new_value.to_uppercase().clone()))
                .returning(FeeStructure::as_returning())
                .get_result(connection)
                .expect("Failed to update expenditure name in fees_structure"),
        ),
        "EXPENDITURE_COST" => {
            let cost_value: i32 = new_value.parse().unwrap();
            Some(
                diesel::update(fees_structure::dsl::fees_structure)
                    .filter(course_name.eq(name_course.to_uppercase().clone()))
                    .filter(year.eq(course_year))
                    .filter(semester.eq(course_sem))
                    .filter(expenditure_name.eq(name_expenditure.to_uppercase().clone()))
                    .set(expenditure_cost.eq(cost_value))
                    .returning(FeeStructure::as_returning())
                    .get_result(connection)
                    .expect("Failed to update expenditure name in fees_structure"),
            )
        }
        _ => {
            println!("Enter a valid field. Either Expenditure_name or Expenditure_cost.");
            None
        }
    }
}
