use crate::connecting::connection_establish::establish_connection;
use crate::course_duration::course_code::course_code_authority::create_course_code_authority::create_course_code_authority;
use crate::fees::fees_authority::create_fees_authority::create_fees_authority;
use crate::models::{CourseCodeAuthority, FeesAuthority};
use crate::student::first_year_authority_init::create_first_year_init_authority::create_first_year_init_authority;
use crate::{
    course_duration::access_to_course_duration::create_allowed_users::create_allowed_users,
    models::{CurriculumAuthority, Staff},
    schema::staff,
    units::curriculum_authority::create_authority_curriculum,
};

use diesel::prelude::*;

/// this is a create lecturer function,
/// the function takes in data of Staff data type
/// inserts into the staff table in the db
/// returns a vector of the update data.
/// The function creates the first year init authority data and prints out the result
/// matches the curriculum authority and only updates the letcurer init authority for cod or allowed users
pub fn create_staff(data: Staff) -> Vec<Staff> {
    let connection = &mut establish_connection();
    let staff_role = data.user_role.clone().to_uppercase();
    let match_staff_role = staff_role.as_str();
    match match_staff_role {
        "COD" => {
            let result = create_first_year_init_authority(
                data.user_id.clone().to_uppercase(),
                data.user_password.clone(),
                data.user_first_name.clone().to_uppercase(),
                data.user_middle_name.clone().to_uppercase(),
                data.user_last_name.clone().to_uppercase(),
                data.user_role.clone(),
            );
            println!("Created: {:?}", result);
            let authority_curriculum_data = CurriculumAuthority {
                owner_id: data.user_id.clone().to_uppercase(),
                owner_password: data.user_password.clone(),
                first_name: data.user_first_name.clone().to_uppercase(),
                middle_name: data.user_middle_name.clone().to_uppercase(),
                last_name: data.user_last_name.clone().to_uppercase(),
                owner_role: data.user_role.clone().to_uppercase(),
            };
            let returned_authority_curriculum =
                create_authority_curriculum(authority_curriculum_data);
            match returned_authority_curriculum {
                Some(curricum_returned_data) => {
                    println!("Returned data: {:?}", curricum_returned_data)
                }
                None => println!("There is no data added in the curriculum authority table"),
            }
            let created_result = create_allowed_users(
                data.user_id.clone().to_uppercase(),
                data.user_password.clone(),
            );
            match created_result {
                Ok(created_data) => println!("Created_data: {created_data:?}"),
                Err(e) => println!("{e:?}"),
            }
            let cca = CourseCodeAuthority {
                user_id: data.user_id.clone().to_uppercase(),
                user_password: data.user_password.clone(),
                user_first_name: data.user_first_name.clone().to_uppercase(),
                user_middle_name: data.user_middle_name.clone().to_uppercase(),
                user_last_name: data.user_last_name.clone().to_uppercase(),
                user_role: data.user_role.clone().to_uppercase(),
            };
            let created_cca = create_course_code_authority(cca);
            println!(
                "Created {:?} because the user is automatically a COD",
                created_cca
            );
            let fees_authority_data = FeesAuthority {
                user_id: data.user_id.to_uppercase().clone(),
                user_password: data.user_password.clone(),
                user_first_name: data.user_first_name.to_uppercase().clone(),
                user_middle_name: data.user_middle_name.clone().to_uppercase(),
                user_last_name: data.user_last_name.to_uppercase().clone(),
                user_role: data.user_role.to_uppercase().clone(),
            };
            let returning_data = create_fees_authority(fees_authority_data);
            println!("Returning data: {returning_data:?}");
        }
        _ => {
            println!("Your role does not allow you to do the update");
        }
    }

    let staff_data = Staff {
        user_id: data.user_id.clone().to_uppercase(),
        user_password: data.user_password.clone(),
        user_first_name: data.user_first_name.clone().to_uppercase(),
        user_middle_name: data.user_middle_name.clone().to_uppercase(),
        user_last_name: data.user_last_name.clone().to_uppercase(),
        user_age: data.user_age.clone(),
        department: data.department.clone().to_uppercase(),
        user_role: data.user_role.clone().to_uppercase(),
        user_position: data.user_position.clone().to_uppercase(),
    };
    let created_data = diesel::insert_into(staff::dsl::staff)
        .values(&staff_data)
        .returning(Staff::as_returning())
        .get_results(connection)
        .expect("Error while inserting data to the lecturer table");
    created_data
}
/*
let data = Staff {
        user_id: String::from("sc/com/0032/23").to_uppercase(),
        user_password: String::from("Skyworth.95"),
        user_first_name: String::from("Stephen").to_uppercase(),
        user_last_name: String::from("Mainda").to_uppercase(),
        user_age: 32,
        department: String::from("Science").to_uppercase(),
        user_role: String::from("Lecturer").to_uppercase(),
        user_position: String::from("COD").to_uppercase(),
    };
*/
