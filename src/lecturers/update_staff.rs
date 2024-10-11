use crate::connecting::connection_establish::establish_connection;
use crate::student::first_year_authority_init::create_first_year_init_authority::create_first_year_init_authority;
use crate::student::first_year_authority_init::update_first_year_init_authority::update_first_year_init_authority;
use crate::{
    lecturers::read_staff::read_one_staff,
    models::{CurriculumAuthority, Staff},
    schema::staff::{
        self, department, user_age, user_first_name, user_id, user_last_name, user_password,
        user_position, user_role,
    },
    units::curriculum_authority::{create_authority_curriculum, update_authority_curriculum},
};
use diesel::prelude::*;

/// update staff method takes user id, field, and new value to be inserted
/// establishes connectin with the database, updates the first_year init: if it exists,
/// the update_first_year_init_authority updates the data but if it fails, just prints out in the screen
/// updates the curriculum authority and if it fails, prints out to the screen
/// The method finally updates the staff database until checking of user role,
/// if the user role, we pass the same arguments to a function that only allows the user's data
/// if the user's role is within the allowed to people to do something, a new data is added to the respective databases
pub fn update_staff(
    user_identity: String,
    field: String,
    new_value: String,
) -> Option<Result<Vec<Staff>, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let modify_field = field.to_uppercase().clone();
    let matched_field = modify_field.as_str();
    let updated_returned_data =
        update_first_year_init_authority(user_identity.clone(), matched_field, new_value.clone());
    match updated_returned_data {
        Some(return_updated_data) => {
            println!("first year init uathority: {:?}", return_updated_data)
        }
        None => {
            println!("Failed to update first year init authority: Returns NONE")
        }
    }
    let curriculum_authority_general_update =
        update_authority_curriculum(user_identity.clone(), new_value.clone(), field.clone());
    match curriculum_authority_general_update {
        Some(result_curriculum_authority) => {
            println!("updates data to: {:?}", result_curriculum_authority)
        }
        None => println!("Failed to update curriculum authority"),
    }
    match matched_field {
        "USER_ID" => Some(
            diesel::update(staff::dsl::staff)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_id.eq(new_value.to_uppercase().clone()))
                .returning(Staff::as_returning())
                .get_results(connection),
        ),
        "USER_PASSWORD" => Some(
            diesel::update(staff::dsl::staff)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_password.eq(new_value.clone()))
                .returning(Staff::as_returning())
                .get_results(connection),
        ),
        "USER_FIRST_NAME" => Some(
            diesel::update(staff::dsl::staff)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_first_name.eq(new_value.to_uppercase().clone()))
                .returning(Staff::as_returning())
                .get_results(connection),
        ),
        "USER_LAST_NAME" => Some(
            diesel::update(staff::dsl::staff)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_last_name.eq(new_value.to_uppercase().clone()))
                .returning(Staff::as_returning())
                .get_results(connection),
        ),
        "USER_AGE" => {
            let converted_age: i32 = new_value.parse().expect("failed to convert the age to ");
            Some(
                diesel::update(staff::dsl::staff)
                    .filter(user_id.eq(user_identity.to_uppercase().clone()))
                    .set(user_age.eq(converted_age))
                    .returning(Staff::as_returning())
                    .get_results(connection),
            )
        }
        "DEPARTMENT" => Some(
            diesel::update(staff::dsl::staff)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(department.eq(new_value.to_uppercase().clone()))
                .returning(Staff::as_returning())
                .get_results(connection),
        ),
        "USER_ROLE" => {
            let stored_value = new_value.clone().to_uppercase();
            let searched_store_value = stored_value.as_str();
            match searched_store_value {
                "COD" => {
                    let user_data_result = read_one_staff(user_identity.to_uppercase().clone());
                    match user_data_result {
                        Ok(user_data) => {
                            let created_data = create_first_year_init_authority(
                                user_data.user_id.clone().to_uppercase(),
                                user_data.user_password.clone(),
                                user_data.user_first_name.clone().to_uppercase(),
                                user_data.user_middle_name.clone().to_uppercase(),
                                user_data.user_last_name.clone().to_uppercase(),
                                new_value.clone().to_uppercase(),
                            );
                            println!("Added  {:?} to first year init authority", created_data);
                            let authority_curriculum_data = CurriculumAuthority {
                                owner_id: user_data.user_id.clone().to_uppercase(),
                                owner_password: user_data.user_password.clone(),
                                first_name: user_data.user_first_name.clone().to_uppercase(),
                                middle_name: user_data.user_middle_name.clone().to_uppercase(),
                                last_name: user_data.user_last_name.clone().to_uppercase(),
                                owner_role: new_value.clone().to_uppercase(),
                            };
                            let curriculum_authority_data =
                                create_authority_curriculum(authority_curriculum_data);
                            match curriculum_authority_data {
                                Some(curriculum_data_authority) => println!(
                                    "Added {:?} to the curriculum authority database",
                                    curriculum_data_authority
                                ),
                                None => println!("Failed to add anything, Returns None"),
                            }
                        }
                        Err(e) => println!("{e:?}"),
                    }
                }
                _ => {
                    println!("You are not allowed to access the system")
                }
            }

            Some(
                diesel::update(staff::dsl::staff)
                    .filter(user_id.eq(user_identity.to_uppercase().clone()))
                    .set(user_role.eq(new_value.to_uppercase().clone()))
                    .returning(Staff::as_returning())
                    .get_results(connection),
            )
        }
        "USER_POSITION" => Some(
            diesel::update(staff::dsl::staff)
                .filter(user_id.eq(user_identity.to_uppercase().clone()))
                .set(user_position.eq(new_value.to_uppercase().clone()))
                .returning(Staff::as_returning())
                .get_results(connection),
        ),
        _ => {
            println!(
                "Please enter a valuable field. either\nuser_id, user_password, user_first_name, user_last_name, user_age, department, user_role, user_position"
            );
            None
        }
    }
}
/*
diesel::table! {
    staff (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_last_name -> Varchar,
        user_age -> Int4,
        department -> Varchar,
        user_role -> Varchar,
        user_position -> Varchar,
    }
}
*/
