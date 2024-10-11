use crate::connecting::connection_establish::establish_connection;
use crate::schema::curriculum::unit_id;

pub fn delete_units(deleted_code: String) -> usize {
    use crate::schema::curriculum::dsl::curriculum as currdsl;
    use diesel::prelude::*;
    let connection = &mut establish_connection();
    let num_deleted_course = diesel::delete(currdsl)
        .filter(unit_id.eq(deleted_code))
        .execute(connection)
        .expect("Error deleting units");
    num_deleted_course
}
