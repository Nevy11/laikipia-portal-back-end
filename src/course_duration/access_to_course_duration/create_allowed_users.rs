use diesel::{RunQueryDsl, SelectableHelper};

use crate::connecting::connection_establish::establish_connection;
use crate::lecturers::read_staff::read_one_staff;
use crate::{models::AccessToCourseDuration, schema::access_to_course_duration};

pub fn create_allowed_users(
    id: String,
    password: String,
) -> Result<AccessToCourseDuration, diesel::result::Error> {
    let conn = &mut establish_connection();
    let id = id.to_uppercase();
    let user_data_result = read_one_staff(id.clone().to_uppercase());
    match user_data_result {
        Ok(user_data) => {
            let user_role = user_data.user_role;
            let data = AccessToCourseDuration {
                id,
                password,
                role: user_role,
            };
            println!("Added the user to Cod\n");
            let result_data =
                diesel::insert_into(access_to_course_duration::dsl::access_to_course_duration)
                    .values(data)
                    .returning(AccessToCourseDuration::as_returning())
                    .get_result(conn);
            result_data
        }
        Err(e) => Err(e),
    }
}
