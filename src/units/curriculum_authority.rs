use diesel::prelude::*;

use crate::connecting::connection_establish::establish_connection;
use crate::{
    models::CurriculumAuthority,
    schema::curriculum_authority::{
        self, first_name, last_name, owner_id, owner_password, owner_role,
    },
};

/// Contains CRUD operation on authority_curriculum db
/// connects and allows the creation of CurriculumAuthority table's id
/// in which the id in the table are allowed to create, modify and delete the
/// units given by the school
/// if the action for creating the data is successfull, it returns an option
/// of The curriculumAuthority struct but if false, returns None
pub fn create_authority_curriculum(data: CurriculumAuthority) -> Option<Vec<CurriculumAuthority>> {
    let conn = &mut establish_connection();
    let response_returned = diesel::insert_into(curriculum_authority::dsl::curriculum_authority)
        .values(&data)
        .returning(CurriculumAuthority::as_returning())
        .get_results(conn)
        .optional();

    match response_returned {
        Ok(data) => data,
        Err(_) => None,
    }
}
/// read
/// returns the data for a particular id.
/// important for checking whether an id is allowed and check the password of
/// that id
/// returns an option of the curriculum struct
/// if it fails, returns None.
pub fn read_one_authority_curriculum(owner_identity: String) -> Option<CurriculumAuthority> {
    let connection = &mut establish_connection();
    let data = curriculum_authority::dsl::curriculum_authority::filter(
        curriculum_authority::table,
        owner_id.eq(owner_identity),
    )
    .select(CurriculumAuthority::as_returning())
    .first(connection)
    .optional();
    println!("{:?}", data);
    match data {
        Ok(Some(return_data)) => {
            println!("Returned: {:?}", return_data);
            Some(return_data)
        }
        Ok(None) => {
            println!("the data is not found");
            None
        }
        Err(_) => {
            println!("Error while fetching data");
            None
        }
    }
}
/// all
/// Gets all the user_name and the password and role, all the data
/// of Curriculum table,
/// only a set of allowed pe
pub fn read_all_authority_curriculum() -> Vec<CurriculumAuthority> {
    let connection = &mut establish_connection();
    let returning_data =
        curriculum_authority::dsl::curriculum_authority::load::<CurriculumAuthority>(
            curriculum_authority::table,
            connection,
        )
        .unwrap_or_else(|err| panic!("failed getting all results in the database:\nError: {err}"));
    returning_data
    // match returning_data {
    //     Ok(Some(result_data)) => Some(result_data),
    //     Ok(None) => {
    //         println!("There is no data in the database");
    //         None
    //     }
    //     Err(err) => {
    //         println!("Error generating all the data: {err}");
    //         None
    //     }
    // }
}
/// update
/// takes in a full Curriculum authority,
/// matches the task based on what needs to be updated
/// returns an option of the value that has been updated
pub fn update_authority_curriculum(
    user_identity: String,
    user_current_data: String,
    task: String,
) -> Option<Vec<CurriculumAuthority>> {
    let connection = &mut establish_connection();

    let update_task = task.to_uppercase();
    let new_updated_task = update_task.as_str();
    match new_updated_task {
        "USER_ID" => {
            let result = diesel::update(curriculum_authority::dsl::curriculum_authority)
            .filter(owner_id.eq(user_identity.to_uppercase().clone()))
                .set(owner_id.eq(user_current_data.to_uppercase().clone()))
                .returning(CurriculumAuthority::as_returning())
                .get_results(connection)
                .unwrap_or_else(|err| {
                    panic!("Failed to update the owner id to the new owner id you have passed:\nError: {err}")
                });
            Some(result)
        }
        "USER_PASSWORD" => {
            let result = diesel::update(curriculum_authority::dsl::curriculum_authority)
                .filter(owner_id.eq(user_identity.to_uppercase().clone()))
                .set(owner_password.eq(user_current_data.clone()))
                .returning(CurriculumAuthority::as_returning())
                .get_results(connection)
                .unwrap_or_else(|err| panic!("Failed to update OWNER PASSWORD:\nError: {err}"));
            Some(result)
        }
        "USER_FIRST_NAME" => {
            let result = diesel::update(curriculum_authority::dsl::curriculum_authority)
                .filter(owner_id.eq(user_identity.to_uppercase().clone()))
                .set(first_name.eq(user_current_data.clone().to_uppercase()))
                .returning(CurriculumAuthority::as_returning())
                .get_results(connection)
                .unwrap_or_else(|err| {
                    panic!("Failed to update the owner's FIRST NAME:\nError: {err}")
                });
            Some(result)
        }
        "USER_LAST_NAME" => {
            let result = diesel::update(curriculum_authority::dsl::curriculum_authority)
                .filter(owner_id.eq(user_identity.to_uppercase().clone()))
                .set(last_name.eq(user_current_data.to_uppercase().clone()))
                .returning(CurriculumAuthority::as_returning())
                .get_results(connection)
                .unwrap_or_else(|err| {
                    panic!("Failed to update the owner's LAST NAME:\nError: {err}");
                });
            Some(result)
        }
        "USER_ROLE" => {
            let result = diesel::update(curriculum_authority::dsl::curriculum_authority)
                .filter(owner_id.eq(user_identity.to_uppercase().clone()))
                .set(owner_role.eq(user_current_data.to_uppercase().clone()))
                .returning(CurriculumAuthority::as_returning())
                .get_results(connection)
                .unwrap_or_else(|err| panic!("Failed to update the user's role:\nError: {err}"));
            Some(result)
        }
        _ => {
            println!("please enter a valid task");
            None
        }
    }
}
/*
pub owner_id: String,
    pub owner_password: String,
    pub first_name: String,
    pub last_name: String,
    pub owner_role: String,
*/

/// delete
/// deletes the data then returns the single instance of the particular data
/// that has been deleted
pub fn delete_one_authority_curriculum(owner_identity: String) -> Option<CurriculumAuthority> {
    let conn = &mut establish_connection();
    let deleted_value = diesel::delete(curriculum_authority::dsl::curriculum_authority)
        .filter(owner_id.eq(owner_identity))
        .returning(CurriculumAuthority::as_select())
        .get_result(conn)
        .optional();

    match deleted_value {
        Ok(Some(data)) => Some(data),
        Ok(None) => {
            println!("The data is not available in curriculum authority table");
            None
        }
        Err(err) => {
            println!("Error deleting the user's id\nError: {err}");
            None
        }
    }
}

/// the function checks to see whether the owner's identity is in the
/// authority curriculum, if it is not, it returns a None which in turns
/// returns false,
/// if the data is available, we compare the user's password entered and that
/// in the database, if it fails, return's success, but if succeed,
/// returns true
pub fn check_for_authority_curriculum(owner_identity: String, owner_to_password: String) -> bool {
    let result = read_one_authority_curriculum(owner_identity);
    match result {
        Some(data) => {
            if data.owner_password == owner_to_password.clone() {
                true
            } else {
                println!("Incorrect password");
                false
            }
        }
        None => {
            println!("The user's id is not available in the database");
            false
        }
    }
}
