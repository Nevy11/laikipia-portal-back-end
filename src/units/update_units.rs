use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::Curriculum,
    schema::curriculum::{self, course_name, electives, unit_id, unit_name},
};

use diesel::prelude::*;

/// updates one of either fields. unit id, unit name, electives, course name
/// user id is given to filter the user to find
/// current value updated is the any new fields of the updated data
/// returns a Some of Data if successfull, but none if fails
pub fn update_units(
    unit_identity: String,
    current_updated_value: String,
    field: &str,
) -> Option<Vec<Curriculum>> {
    let connection = &mut establish_connection();

    let field = field.to_uppercase().clone();
    let search_field = field.as_str();
    match search_field {
        "UNIT_ID" => {
            let updated_data = diesel::update(curriculum::dsl::curriculum)
                .filter(unit_id.eq(unit_identity.to_uppercase().clone().to_uppercase()))
                .set(unit_id.eq(current_updated_value.clone().to_uppercase()))
                .returning(Curriculum::as_select())
                .get_results(connection)
                .expect("Unable to update the unit id");
            Some(updated_data)
        }
        "UNIT_NAME" => {
            let updated_data = diesel::update(curriculum::dsl::curriculum)
                .filter(unit_id.eq(unit_identity.to_uppercase().clone().to_uppercase()))
                .set(unit_name.eq(current_updated_value.clone().to_uppercase()))
                .returning(Curriculum::as_select())
                .get_results(connection)
                .expect("Unable to update the unit Name");
            Some(updated_data)
        }
        "ELECTIVES" => {
            let updated_data = diesel::update(curriculum::dsl::curriculum)
                .filter(unit_id.eq(unit_identity.to_uppercase().clone().to_uppercase()))
                .set(electives.eq(current_updated_value.clone().to_uppercase()))
                .returning(Curriculum::as_select())
                .get_results(connection)
                .expect("Unable to update the electives");
            Some(updated_data)
        }
        "COURSE_NAME" => {
            let updated_data = diesel::update(curriculum::dsl::curriculum)
                .filter(unit_id.eq(unit_identity.to_uppercase().clone().to_uppercase()))
                .set(course_name.eq(current_updated_value.clone().to_uppercase()))
                .returning(Curriculum::as_select())
                .get_results(connection)
                .expect("Unable to update the course name");
            Some(updated_data)
        }
        _ => {
            println!(
                "Please enter a valid field, either: unit_id, unit_name, electives or course name!"
            );
            None
        }
    }
}
