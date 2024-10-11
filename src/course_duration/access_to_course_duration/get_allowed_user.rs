use crate::connecting::connection_establish::establish_connection;
use crate::{models::AccessToCourseDuration, schema::access_to_course_duration};
use diesel::prelude::*;

pub fn get_allowed_user(reg_no: String) -> AccessToCourseDuration {
    let connection = &mut establish_connection();
    let result_data_access = access_to_course_duration::dsl::access_to_course_duration
        .find(reg_no.to_uppercase().clone())
        .select(AccessToCourseDuration::as_returning())
        .get_result(connection)
        .expect("failed loading the user value");

    result_data_access
}
pub fn get_all_allowed_user() -> Vec<AccessToCourseDuration> {
    let connection = &mut establish_connection();
    let result = access_to_course_duration::dsl::access_to_course_duration::load::<
        AccessToCourseDuration,
    >(access_to_course_duration::table, connection)
    .expect("failed to return the course duration data");
    result
}

pub fn check_authorisation_for_allowed_user(
    owner_identity: String,
    owner_password: String,
) -> bool {
    let owner_data = get_allowed_user(owner_identity.clone().to_uppercase());
    if owner_data.password == owner_password {
        true
    } else {
        false
    }
}
