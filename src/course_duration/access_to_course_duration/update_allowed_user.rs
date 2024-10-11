use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::AccessToCourseDuration,
    schema::access_to_course_duration::{self, id, password, role},
};

use diesel::prelude::*;

pub fn update_allowed_user(
    user_id: String,
    new_value: String,
    field: String,
) -> Option<Result<AccessToCourseDuration, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let search_field = field.as_str();
    println!("update_allowed_user is called");
    match search_field {
        "USER_ID" => {
            let result = diesel::update(access_to_course_duration::dsl::access_to_course_duration)
                .filter(id.eq(user_id.to_uppercase()))
                .set(id.eq(new_value.to_uppercase()))
                .get_result(connection);
            Some(result)
        }
        "USER_PASSWORD" => {
            let result = diesel::update(access_to_course_duration::dsl::access_to_course_duration)
                .filter(id.eq(user_id.to_uppercase()))
                .set(password.eq(new_value))
                .get_result(connection);
            Some(result)
        }
        "USER_ROLE" => {
            let result = diesel::update(access_to_course_duration::dsl::access_to_course_duration)
                .filter(id.eq(user_id.to_uppercase()))
                .set(role.eq(new_value.to_uppercase()))
                .get_result(connection);
            Some(result)
        }
        _ => {
            println!("Please enter a valid role");
            None
        }
    }
}
