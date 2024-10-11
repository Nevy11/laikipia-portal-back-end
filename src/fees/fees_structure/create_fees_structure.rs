use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection, models::FeeStructure,
    schema::fees_structure,
};

/// The function creates a new fee structure data.
/// Adds the course name, year, sem, expe_name and expe_cost to the fee_structure data.
/// Returns the data as FeeStructur struct.
pub fn create_fees_structure(data: FeeStructure) -> FeeStructure {
    let connection = &mut establish_connection();
    let created_data = FeeStructure {
        course_name: data.course_name.to_uppercase().clone(),
        year: data.year,
        semester: data.semester,
        expenditure_name: data.expenditure_name.to_uppercase().clone(),
        expenditure_cost: data.expenditure_cost,
    };
    diesel::insert_into(fees_structure::dsl::fees_structure)
        .values(created_data)
        .returning(FeeStructure::as_returning())
        .get_result(connection)
        .expect("Failed to insert the data to the fee_structure table")
}
