use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::TotalGraduation,
    schema::total_graduation::{
        self, fail, first_class_students, graduation_year, number_students, pass,
        second_class_lower_division_students, second_class_upper_division_students,
    },
};

/// The methods takes in the year of graduation, field to update, and the new value in a form of i32 since all fields are in the form of
/// i32. If the field does not match any of the column, it returns None. If the updates goes successfully or not, it returns a result of
/// the total graduation.
pub fn update_total_graduation(
    year_of_graduation: i32,
    field: String,
    new_value: i32,
) -> Option<Result<TotalGraduation, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field_value = field.as_str();
    match field_value {
        "GRADUATION_YEAR" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(graduation_year.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        "NUMBER_STUDENTS" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(number_students.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        "FIRST_CLASS_STUDENTS" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(first_class_students.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        "SECOND_CLASS_UPPER_DIVISION_STUDENTS" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(second_class_upper_division_students.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        "SECOND_CLASS_LOWER_DIVISION_STUDENTS" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(second_class_lower_division_students.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        "PASS" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(pass.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        "FAIL" => Some(
            diesel::update(total_graduation::dsl::total_graduation)
                .filter(graduation_year.eq(year_of_graduation))
                .set(fail.eq(new_value))
                .returning(TotalGraduation::as_returning())
                .get_result(connection),
        ),
        _ => {
            println!(
                "
                Please enter a valid column name to make the changes. Either: \n
                graduation_year,
                \nnumber_students,
                \nfirst_class_students,
                \n second_class_upper_division_students,
                \n second_class_lower_division_students,
                \n pass,
                \n fail
            "
            );
            None
        }
    }
}
