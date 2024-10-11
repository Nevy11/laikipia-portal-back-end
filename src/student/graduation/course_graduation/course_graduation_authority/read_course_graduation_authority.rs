use diesel::prelude::*;

use crate::{
    connecting::connection_establish::establish_connection,
    models::CourseGraduationAuthority,
    schema::{course_graduation_authority, course_graduation_authority::user_id},
};

/// Thsi method takes in the user identity returning a result of the user's data in a CourseGraduationAuthority struct
/// filters the data, selecting the struct after establishing the connection with the database.
pub fn read_one_course_graduation_authority(
    user_identity: String,
) -> Result<CourseGraduationAuthority, diesel::result::Error> {
    let connection = &mut establish_connection();
    course_graduation_authority::dsl::course_graduation_authority
        .filter(user_id.eq(user_identity.to_uppercase()))
        .select(CourseGraduationAuthority::as_returning())
        .get_result(connection)
}

/// The method reads all the data and returns it in a form of a Result of the vector of CourseGraduationAuthority
/// struct.
pub fn read_all_course_graduation_authority(
) -> Result<Vec<CourseGraduationAuthority>, diesel::result::Error> {
    let connection = &mut establish_connection();
    course_graduation_authority::dsl::course_graduation_authority::load::<CourseGraduationAuthority>(
        course_graduation_authority::table,
        connection,
    )
}

/// The method takes in the user identity into which it reads its data from the database. If it is available, it returns its
/// data as a struct but if unavailable, It returns an error. In case of any error, the method returns false, but if there is no error,
/// the password of the user stored in the database is compared to that passed to the function; if it matches, returns true but if it doesn't,
/// returns false
pub fn check_course_graduation_authority_passkey(
    user_identity: String,
    password_of_user: String,
) -> bool {
    let user_result = read_one_course_graduation_authority(user_identity.to_uppercase().clone());
    match user_result {
        Ok(user_data) => {
            if user_data.user_password == password_of_user {
                true
            } else {
                println!("incorrect password");
                false
            }
        }
        Err(e) => {
            println!("{e:?}");
            false
        }
    }
}
