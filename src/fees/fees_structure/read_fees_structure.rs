use crate::{
    connecting::connection_establish::establish_connection,
    models::{
        CourseExpenditureData, FeeStructure, FeeStructureLoadAll, SemExpenditureData,
        YearExpenditureData,
    },
    schema::fees_structure::{self, course_name, expenditure_name, semester, year},
};
use diesel::prelude::*;

/// the function reads one course expenditure and returns the cost of the expenditure in the fees_structure table
/// It takes course name, year, semester and name of the expenditure
/// filters the data based on the year, course_name, sem and expenditure name holding the entire data for the expenditure
/// Returns the expenditure cost
pub fn read_one_expenditure_cost(
    name_course: String,
    course_year: i32,
    course_sem: i32,
    name_expenditure: String,
) -> i32 {
    let connection = &mut establish_connection();
    let expenditure_data = fees_structure::dsl::fees_structure
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .filter(year.eq(course_year))
        .filter(semester.eq(course_sem))
        .filter(expenditure_name.eq(name_expenditure.to_uppercase().clone()))
        .select(FeeStructure::as_returning())
        .get_result(connection)
        .expect("Failed getting expenditure data in fees_structure table");
    expenditure_data.expenditure_cost
}

/// This function reads one semester expenditure cost returning the expenditure name and cost of that semester
/// As usual, filters the data in the fees_structure table
pub fn read_one_sem_expenditure_cost(
    name_course: String,
    course_year: i32,
    course_sem: i32,
) -> SemExpenditureData {
    let connection = &mut establish_connection();
    let expenditure_data = fees_structure::dsl::fees_structure
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .filter(year.eq(course_year))
        .filter(semester.eq(course_sem))
        .select(FeeStructure::as_returning())
        .get_result(connection)
        .expect("Failed getting semester expenditure data in fees_structure table");
    let data = SemExpenditureData {
        expenditure_name: expenditure_data.expenditure_name,
        expenditure_cost: expenditure_data.expenditure_cost,
    };
    data
}

/// Reads one year expenditure cost from the fees structure table in the database.
pub fn read_one_year_expenditure_cost(
    name_course: String,
    course_year: i32,
) -> YearExpenditureData {
    let connection = &mut establish_connection();
    let expenditure_data = fees_structure::dsl::fees_structure
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .filter(year.eq(course_year))
        .select(FeeStructure::as_returning())
        .get_result(connection)
        .expect("Failed getting semester expenditure data in fees_structure table");
    let data = YearExpenditureData {
        course_sem: expenditure_data.semester,
        expenditure_name: expenditure_data.expenditure_name,
        expenditure_cost: expenditure_data.expenditure_cost,
    };
    data
}

/// Returns all the expenditure data of a particular course
pub fn read_one_course_expenditure_cost(name_course: String) -> CourseExpenditureData {
    let connection = &mut establish_connection();
    let expenditure_data = fees_structure::dsl::fees_structure
        .filter(course_name.eq(name_course.to_uppercase().clone()))
        .select(FeeStructure::as_returning())
        .get_result(connection)
        .expect("Failed getting semester expenditure data in fees_structure table");
    let data = CourseExpenditureData {
        course_year: expenditure_data.year,
        course_sem: expenditure_data.semester,
        expenditure_name: expenditure_data.expenditure_name,
        expenditure_cost: expenditure_data.expenditure_cost,
    };
    data
}

/// Returns all the course expenditure data in form of a vector.
/// Takes nothing, establishes the connection then returns the entire data in a form of a vector of  FeeStructureLoadAll
pub fn read_all_course_expenditure() -> Vec<FeeStructureLoadAll> {
    let connection = &mut establish_connection();
    let all_exp_data = fees_structure::dsl::fees_structure::load::<FeeStructureLoadAll>(
        fees_structure::table,
        connection,
    )
    .expect("Failed to load all the data from the fee structure table");
    all_exp_data
}
