use diesel::{RunQueryDsl, SelectableHelper};

use crate::connecting::connection_establish::establish_connection;
use crate::{models::Curriculum, schema::curriculum};

/// takes in the unit name, unit code, and electives
/// creates the connection with the database
/// stores the data in AllUnits struct that takes in unit name and code
/// It inserts into all units table in the database
/// updates the curriculum database too.
///
pub fn units_id_reg(
    unit_name: String,
    unit_code: String,
    electives: String,
    course_name: String,
) -> Vec<Curriculum> {
    let connection = &mut establish_connection();

    // curriculum
    let curriculum_data = Curriculum {
        unit_id: unit_code.clone(),
        unit_name: unit_name.clone().to_uppercase(),
        electives: electives.to_uppercase().clone(),
        course_name: course_name.to_uppercase().clone(),
    };
    let result = diesel::insert_into(curriculum::table)
        .values(&curriculum_data)
        .returning(Curriculum::as_returning())
        .get_results(connection)
        .expect("error inserting data in curriculum table");
    result
}
