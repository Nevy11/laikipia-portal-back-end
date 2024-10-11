use crate::{
    connecting::connection_establish::establish_connection,
    course_duration::course_code::course_code_authority::delete_course_code_authority::delete_course_code_authority,
    student::first_year_authority_init::delete_first_year_init_authority::main_delete_user_in_first_year_init_authority,
};
use diesel::prelude::*;

use crate::{
    course_duration::access_to_course_duration::delete_allowed_user::delete_allowed_user,
    models::Staff,
    schema::staff::{self, user_id},
    units::curriculum_authority::delete_one_authority_curriculum,
};

/// delete staff function takes in a user_identity the removes the user from the first year init authority
/// deletes the data from the authority curriculum database
/// deletes the data from the staff
pub fn delete_staff(user_identity: String) -> Vec<Staff> {
    let connection = &mut establish_connection();
    let deleted_init_data =
        main_delete_user_in_first_year_init_authority(user_identity.to_uppercase().clone());
    match deleted_init_data {
        Some(init_deleted_data) => println!(
            "delete_user_in_first_year_init_authority{:?}",
            init_deleted_data
        ),
        None => println!("There is no data to delete"),
    }
    let one_authority_curriculum =
        delete_one_authority_curriculum(user_identity.to_uppercase().clone());
    match one_authority_curriculum {
        Some(returned_authority_curriculum) => println!(
            "delete_one_authority_curriculum: {:?}",
            returned_authority_curriculum
        ),
        None => println!("There is no data to delete"),
    }
    let deleted_data = delete_allowed_user(user_identity.clone().to_uppercase());
    println!("Deleted_allowed_course duration data: {:?}", deleted_data);
    let deleted_data = delete_course_code_authority(user_identity.clone().to_uppercase());
    match deleted_data {
        Some(code_deleted_data) => println!("The deleted data is: {code_deleted_data:?}"),
        None => println!("Failed to delete Course code authority data"),
    }
    let deleted_data = diesel::delete(staff::dsl::staff)
        .filter(user_id.eq(user_identity.to_uppercase().clone()))
        .returning(Staff::as_select())
        .get_results(connection)
        .expect("failed to delete the user from the system");
    deleted_data
}
