use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::AccessToCourseDuration,
    schema::access_to_course_duration::{self, id},
};

pub fn delete_allowed_user(user_identity: String) -> AccessToCourseDuration {
    let connection = &mut establish_connection();
    let result = diesel::delete(access_to_course_duration::dsl::access_to_course_duration)
        .filter(id.eq(user_identity))
        .returning(AccessToCourseDuration::as_returning())
        .get_result(connection)
        .expect("Failed to delete the user identity from Acces sto course duration ");
    result
}
