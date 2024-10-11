use actix_cors::Cors;
use actix_web::{
    delete, get, http, patch, post, web::Json, App, HttpResponse, HttpServer, Responder,
};
use course_duration::{
    access_to_course_duration::{
        create_allowed_users::create_allowed_users,
        delete_allowed_user::delete_allowed_user,
        get_allowed_user::{
            check_authorisation_for_allowed_user, get_all_allowed_user, get_allowed_user,
        },
        update_allowed_user::update_allowed_user,
    },
    course_code::{
        course_code_authority::{
            create_course_code_authority::create_course_code_authority,
            delete_course_code_authority::delete_course_code_authority,
            read_course_code_authority::{
                check_course_code_authority_password, read_all_course_code_authority,
                read_one_course_code_authority,
            },
            update_course_code_authority::update_course_code_authority,
        },
        create_course_code::create_course_code,
        delete_course_code::delete_course_code,
        read_course_code::{read_all_course_codes, read_one_course_code},
        update_course_code::update_course_code,
    },
    create_course_duration::create_course_in_course_duration_table,
    delete_course_duration::delete_a_course_in_course_duration_table,
    read_course_duration::{
        get_all_courses_in_course_duration_table, get_course_in_course_duration_table,
    },
    update_course_duration::update_course_in_course_duration_table,
};
use fees::{
    fees_authority::{
        create_fees_authority::create_fees_authority,
        delete_fees_authority::delete_fees_authority,
        read_fees_authority::{
            check_password_fees_authority, read_all_fees_authority, read_one_fees_authority,
        },
        update_fees_authority::update_fees_authority,
    },
    fees_structure::{
        create_fees_structure::create_fees_structure,
        delete_fees_structure::delete_expenditure_fee_structure,
        read_fees_structure::{
            read_one_course_expenditure_cost, read_one_expenditure_cost,
            read_one_sem_expenditure_cost, read_one_year_expenditure_cost,
        },
        update_fees_structure::update_ependiture_fees_structure,
    },
};
use hostels::{
    create_hostel::create_hostel,
    delete_hostel::delete_hostel,
    hostels_authority::{
        create_hostels_authority::create_hostels_authority,
        delete_hostels_authority::delete_hostels_authority,
        read_hostels_authority::{
            check_hostel_authority_authorization, read_all_hostel_authority,
            read_one_hostel_authority,
        },
        update_hostels_authority::update_hostels_authority,
    },
    read_hostel::{read_all_hostels, read_one_hostel},
    student_hostels::{
        create_student_hostel::create_student_hostel, delete_student_hostel::delete_student_hostel,
        read_student_hostel::read_one_student_hostel, update_student_hostel::update_student_hostel,
    },
    update_hostel::update_hostel,
};
use lecturers::{
    create_staff::create_staff,
    delete_staff::delete_staff,
    read_staff::{check_for_staff_authorization, read_all_staff, read_one_staff},
    update_staff::update_staff,
};
use models::{
    AllowedCourseDuration, AuthorityGraduation, CourseCodeAuthority, CourseCodeAuthorityCreate,
    CourseCodeAuthorityDelete, CourseCodeAuthorityReadAll, CourseCodeAuthorityReadOne,
    CourseCodeAuthorityUpdate, CourseCodeDelete, CourseCodeReadOne, CourseCodeTakeIn,
    CourseCodeUpdate, CourseDurationGetOne, CourseGraduation, CourseGraduationAuthority,
    CourseGraduationAuthorityCreate, CourseGraduationAuthorityGetAll,
    CourseGraduationAuthorityGetOne, CourseGraduationAuthorityReturn,
    CourseGraduationAuthorityUpdate, CourseGraduationCreate, CourseGraduationDelete,
    CourseGraduationGetOne, CourseGraduationUpdate, CreateCourseDuration,
    CreateCourseDurationAuthority, CreateGraduationAuthority, CurriculumAuthorityUpdate,
    CurriculumEntrance, CurriculumReadAllAuthority, CurriculumReadOneAuthority,
    DeleteCourseDuration, DeleteCourseDurationAuthority, DeleteGraduationAuthority, DeleteUnitId,
    FeeStructure, FeeStructureCreate, FeeStructureDelete, FeeStructureGetOne,
    FeeStructureGetOneExpenditure, FeeStructureGetOneSem, FeeStructureGetOneYear,
    FeeStructureUpdate, FeesAuthority, FeesAuthorityCreate, FeesAuthorityDelete,
    FeesAuthorityReadAll, FeesAuthorityReadOne, FeesAuthorityUpdate, FirstInitGetAllAuthority,
    GetAllGraduationAuthority, GetOneCourseDuration, GetOneGraduationAuthority,
    GraduationStudentCreate, GraduationStudentReadAll, GraduationStudentReadOne, Hostels,
    HostelsAuthority, HostelsAuthorityCreate, HostelsAuthorityReadAll, HostelsAuthorityReadOne,
    HostelsAuthorityUpdate, HostelsCreate, HostelsDelete, HostelsGetOne, HostelsUpdate, LoginData,
    OneExpenditureCost, OneStudentData, RemoveStudentsData, ReturnSignUpStudent, SignUpStudent,
    Staff, StaffDelete, StaffReadAll, StaffReadOne, StaffUpdate, StudentHostel, Students,
    StudentsDataAll, StudentsGraduation, StudentsHostelsCreate, StudentsHostelsReadOne,
    StudentsHostelsUpdate, TotalGraduation, TotalGraduationCreate, TotalGraduationGetAll,
    TotalGraduationGetOne, TotalGraduationUpdate, Update, UpdateAllowedCourseDurationCurriculum,
    UpdateCourseDuration, UpdateGraduationAuthority, UpdateStaff,
};

use student::{
    create_student::first_years_init,
    delete_student::delete_student,
    first_year_authority_init::read_first_year_init_authority::{
        check_first_year_init_existance, get_all_first_year_init_authority,
    },
    graduation::{
        authority_graduation::{
            create_authority_graduation::create_authority_graduation,
            delete_authority_graduation::delete_authority_graduation,
            read_authority_graduation::{
                check_authority_graduation_passkey, read_all_authority_graduation,
                read_one_authority_graduation,
            },
            update_authority_graduation::update_authority_graduation,
        },
        course_graduation::{
            course_graduation_authority::{
                create_course_graduation_authority::create_course_graduation_authority,
                delete_course_graduation_authority::delete_course_graduation_authority,
                read_course_graduation_authority::{
                    check_course_graduation_authority_passkey,
                    read_all_course_graduation_authority, read_one_course_graduation_authority,
                },
                update_course_graduation_authority::updated_course_graduation_authority,
            },
            create_course_graduation::create_course_graduation,
            delete_course_graduation::delete_course_graduation,
            read_course_graduation::{read_all_course_graduation, read_one_course_graduation},
            total_graduation::{
                create_total_graduation::create_total_graduation,
                delete_total_graduation::delete_total_graduation,
                read_total_graduation::{
                    read_all_total_graduation_data, read_one_total_graduation_data,
                },
                update_total_graduation::update_total_graduation,
            },
            update_course_graduation::update_course_graduation,
        },
        students_graduation::{
            create_students_graduation::create_students_graduation,
            delete_students_graduation::delete_students_graduation,
            read_students_graduation::{
                read_all_students_graduation, read_one_students_graduation,
            },
        },
    },
    read_student::{check_student_password, read_all_student, read_one_student},
    set_student_sem_year::set_student_sem_year,
};
use time::{current_day::current_day, current_month::current_month, current_year::current_year};
use time_practice::get_time_now::get_time_now;
use units::{
    curriculum_authority::{
        check_for_authority_curriculum, read_all_authority_curriculum,
        read_one_authority_curriculum, update_authority_curriculum,
    },
    delete_units::delete_units,
    get_units::get_units,
    units_id_reg::units_id_reg,
    update_units::update_units,
};

pub mod connecting;
pub mod course_duration;
pub mod fees;
pub mod hostels;
pub mod lecturers;
pub mod models;
pub mod schema;
pub mod student;
pub mod time;
pub mod time_practice;
pub mod units;

#[post("/login")]
pub async fn login_student(data: Json<LoginData>) -> impl Responder {
    let is_allowed =
        check_student_password(data.reg_no.clone().to_uppercase(), data.password.clone());
    if is_allowed {
        let data_result =
            set_student_sem_year(data.reg_no.clone().to_uppercase(), data.password.clone());
        match data_result{
            Some(result) => HttpResponse::Ok().json(result),
            None =>{
                HttpResponse::Ok().body("Failed to load user login service. Either the student is removed from the system, or incorrect password")
        }
    }
    } else {
        HttpResponse::Ok().body("Not allowed to use the system")
    }
}

#[post("/students/sign_up")]
pub async fn sign_up_student(data: Json<SignUpStudent>) -> impl Responder {
    let allowed = check_first_year_init_existance(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    let adm_date: i32 = current_day()
        .try_into()
        .expect("Failed to update the admission date to i32");
    let adm_month: i32 = current_month()
        .try_into()
        .expect("Failed to convert the current month to i32");
    if allowed {
        let student_data = Students {
            reg_no: data.reg_no.clone().to_uppercase(),
            password: data.password.clone(),
            first_name: data.first_name.clone().to_uppercase(),
            middle_name: data.middle_name.clone().to_uppercase(),
            last_name: data.last_name.clone().to_uppercase(),
            year_of_study: data.year_of_study.clone(),
            semester: data.semester.clone(),
            course: data.course.clone().to_uppercase(),
            programme: data.programme.clone().to_uppercase(),
            department: data.department.clone().to_uppercase(),
            school: data.school.to_uppercase().clone(),
            class: data.class.clone().to_uppercase(),
            gssp: data.gssp.clone().to_uppercase(),
            students_role: data.students_role.clone().to_uppercase(),
            admission_date: adm_date,
            admission_month: adm_month,
            admission_year: current_year(),
            gender: data.gender.clone(),
        };
        let result = first_years_init(
            student_data.reg_no,
            student_data.password,
            student_data.first_name,
            student_data.last_name,
            student_data.middle_name,
            student_data.year_of_study,
            student_data.semester,
            student_data.programme,
            student_data.course,
            student_data.department,
            student_data.school,
            student_data.class,
            student_data.gssp,
            student_data.students_role,
            student_data.gender,
            student_data.admission_date.try_into().unwrap(),
            student_data.admission_month.try_into().unwrap(),
            student_data.admission_year,
        );
        let returning_data = ReturnSignUpStudent {
            reg_no: result.reg_no,
            first_name: result.first_name,
            middle_name: result.middle_name,
            last_name: result.last_name,
            year_of_study: result.year_of_study,
            semester: result.semester,
            course: result.course,
            programme: result.programme,
            department: result.department,
            school: result.school,
            class: result.class,
            gssp: result.gssp,
            students_role: result.students_role,
            admission_date: result.admission_date,
            admission_month: result.admission_month,
            admission_year: result.admission_year,
        };
        // naffa kuadd first year student kwa database
        HttpResponse::Ok().json(returning_data)
    } else {
        HttpResponse::Ok().body("Incorrect username or password")
    }
}

#[get("/students/students_data_all")]
pub async fn get_students_data_all(data: Json<StudentsDataAll>) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let role_owner = owner_data.user_role.as_str();
                match role_owner {
                    "COD" => {
                        let value = read_all_student();
                        HttpResponse::Ok().json(value)
                    }
                    _ => HttpResponse::Ok()
                        .body("Your current role does not allow you to get all the student's data"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("incorrect user name or password")
    }
}

#[get("/students/one_students_data")]
pub async fn get_one_students_data(data: Json<OneStudentData>) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let role_owner = owner_data.user_role.as_str();
                match role_owner {
                    "COD" => {
                        let value = read_one_student(data.reg_no.clone().to_uppercase());
                        HttpResponse::Ok().json(value)
                    }
                    _ => HttpResponse::Ok()
                        .body("Your current role does not allow you to get all the student's data"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Not authorised to get the students data")
    }
}

#[delete("/students/remove_student")]
pub async fn remove_students_data_all(data: Json<RemoveStudentsData>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let role_owner = owner_data.user_role.to_uppercase();
                let role_matched = role_owner.as_str();
                match role_matched {
                    "COD" => {
                        let deleted_data = delete_student(data.reg_no.clone().to_uppercase());
                        HttpResponse::Ok().json(deleted_data)
                    }
                    _ => {
                        println!("Your role does not allow you to use the house");
                        HttpResponse::Ok().body("None")
                    }
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[post("/first_init/get_all_authority")]
pub async fn get_all_authority_first_year_init(
    data: Json<FirstInitGetAllAuthority>,
) -> impl Responder {
    let user_identity = data.owner_id.clone().to_uppercase();
    let user_password = data.owner_password.clone();
    let role = data.role.to_uppercase();
    let search_role = role.as_str();
    match search_role {
        "COD" => {
            check_first_year_init_existance(user_identity, user_password);
            let result = get_all_first_year_init_authority();
            HttpResponse::Ok().json(result)
        }
        _ => HttpResponse::Ok().body(format!(
            "You are not allowed to view all the data in the system"
        )),
    }
}

#[post("/staff/add_staff")]
pub async fn sign_up_staff(data: Json<StaffUpdate>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let role_of_owner = owner_data.user_role.clone().to_uppercase();
                let search_role_owner = role_of_owner.as_str();
                match search_role_owner {
                    "COD" => {
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
                        let returning_data = create_staff(staff_data);
                        HttpResponse::Ok().json(returning_data)
                    }
                    _ => HttpResponse::Ok().body("You have no authorization to access the staff"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("incorrect owner username or password")
    }
}

#[delete("/staff/delete_staff")]
pub async fn staff_delete(data: Json<StaffDelete>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let role_of_owner = owner_data.user_role.clone().to_uppercase();
                let search_role_owner = role_of_owner.as_str();
                match search_role_owner {
                    "COD" => {
                        let user_identity = data.user_id.to_uppercase().clone();
                        let deleted_data = delete_staff(user_identity);
                        HttpResponse::Ok().json(deleted_data)
                    }
                    _ => HttpResponse::Ok().body("You are not authorised to access the system"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("incorrect owner password")
    }
}
#[get("/staff/readAllStaff")]
pub async fn staff_read_all(data: Json<StaffReadAll>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let role_of_owner = owner_data.user_role.clone().to_uppercase();
                let search_role_owner = role_of_owner.as_str();
                match search_role_owner {
                    "COD" => {
                        let data_returned = read_all_staff();
                        HttpResponse::Ok().json(data_returned)
                    }
                    _ => HttpResponse::Ok().body("Not authorised to access this system"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("incorrect owner password")
    }
}

#[get("/staff/readOneStaff")]
pub async fn staff_read_one(data: Json<StaffReadOne>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.user_id.to_uppercase().clone(),
        data.user_password.clone(),
    );
    if is_allowed {
        let data_returned_result = read_one_staff(data.user_id.clone().to_uppercase());
        match data_returned_result {
            Ok(data_returned) => HttpResponse::Ok().json(data_returned),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect password")
    }
}

#[patch("/staff/updateStaff")]
pub async fn staff_update(data: Json<UpdateStaff>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let role_of_owner = owner_data.user_role.clone().to_uppercase();
                let search_role_owner = role_of_owner.as_str();
                match search_role_owner {
                    "COD" => {
                        let returned_update_staff_result = update_staff(
                            data.user_id.clone(),
                            data.field.clone(),
                            data.new_value.clone(),
                        );
                        match returned_update_staff_result {
                            Some(Ok(updated_data)) => HttpResponse::Ok().json(updated_data),
                            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
                            None => HttpResponse::Ok().body("select a valid field to update"),
                        }
                    }
                    _ => HttpResponse::Ok().body("You are not allowed to use this system"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("incorrect owner password")
    }
}

#[post("/curriculum/read_all_curriculum_authority")]
pub async fn read_authority_curriculum_all(
    data: Json<CurriculumReadAllAuthority>,
) -> impl Responder {
    let is_allowed = check_for_authority_curriculum(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_s_data = read_one_authority_curriculum(data.owner_id.clone().to_uppercase());
        match owner_s_data {
            Some(data) => {
                let owner_s_role = data.owner_role.clone().to_uppercase();
                let role_of_owner = owner_s_role.as_str();
                match role_of_owner {
                    "COD" => {
                        let all_data = read_all_authority_curriculum();
                        HttpResponse::Ok().json(all_data)
                    }
                    _ => {
                        HttpResponse::Ok().body("Your position does not allow you to read all data")
                    }
                }
            }
            None => HttpResponse::Ok().body(format!("data is not found in the db")),
        }
    } else {
        HttpResponse::Ok().body("Not allowed to read all curriculum authority data")
    }
}

#[post("/curriculum/read_one_curriculum_authority")]
pub async fn read_authority_curriculum_one(
    data: Json<CurriculumReadOneAuthority>,
) -> impl Responder {
    let is_allowed = check_for_authority_curriculum(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data = read_one_authority_curriculum(data.owner_id.clone().to_uppercase());
        match owner_data {
            Some(returning_data) => {
                let owner_data_role = returning_data.owner_role.clone();
                let search_role = owner_data_role.as_str();
                match search_role {
                    "COD" => {
                        let data_returned =
                            read_one_authority_curriculum(data.user_id.clone().to_uppercase());
                        match data_returned {
                            Some(found_data) => HttpResponse::Ok().json(found_data),
                            None => {
                                HttpResponse::Ok().body("Data you are looking for is not found")
                            }
                        }
                    }
                    _ => HttpResponse::Ok()
                        .body("Your rank does not allow you to view the user's data"),
                }
            }
            None => HttpResponse::Ok().body("Error while collecting the user's database issue"),
        }
    } else {
        HttpResponse::Ok().body("Incorrect password")
    }
}

#[patch("/curriculum/updates_curriculum_authority")]
pub async fn updates_curriculum_authority(data: Json<CurriculumAuthorityUpdate>) -> impl Responder {
    let is_allowed = check_for_authority_curriculum(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let result_data = read_one_authority_curriculum(data.owner_id.clone().to_uppercase());

        match result_data {
            Some(owner_curriculum_data) => {
                let owner_role = owner_curriculum_data.owner_role.clone();
                let search_owner_role = owner_role.as_str();
                match search_owner_role {
                    "COD" => {
                        let returning_update_data = update_authority_curriculum(
                            data.user_identity.clone(),
                            data.user_current_data.clone(),
                            data.field.clone().to_uppercase(),
                        );
                        match returning_update_data {
                            Some(update_data_returned) => {
                                HttpResponse::Ok().json(update_data_returned)
                            }
                            None => {
                                HttpResponse::Ok().body(format!("Failed to update Owner_id data"))
                            }
                        }
                    }

                    _ => HttpResponse::Ok()
                        .body(format!("You're rank does not allow you to use the system")),
                }
            }
            None => HttpResponse::Ok().body("Data is not found for the owner"),
        }
    } else {
        HttpResponse::Ok().body("The user is not allowed to use the system")
    }
}
/// post all units api
/// takes in the course_id, course_name and electives
/// creates a new unit based on the user.. who is allowed to do this
/// operation depending on his or her allowed acess table
#[post("/curriculum/add_unit")]
pub async fn all_units_reg(body: Json<CurriculumEntrance>) -> impl Responder {
    let is_allowed = check_for_authority_curriculum(
        body.owner_id.clone().to_uppercase(),
        body.owner_password.clone(),
    );
    if is_allowed {
        let owner_s_data = read_one_authority_curriculum(body.owner_id.clone().to_uppercase());
        match owner_s_data {
            Some(owner_s_data_matched) => {
                let owner_s_role = owner_s_data_matched.owner_role.clone();
                let search_owner_s_role = owner_s_role.as_str();
                match search_owner_s_role {
                    "COD" => {
                        let unit_name = body.unit_name.clone();
                        let unit_code = body.unit_id.clone();
                        let electives = body.electives.clone();
                        let course_name = body.course_name.clone();

                        let data = units_id_reg(
                            unit_name.to_string().to_uppercase().clone(),
                            unit_code.to_string().to_uppercase().clone(),
                            electives.to_string().to_uppercase().clone(),
                            course_name.to_string().to_uppercase().clone(),
                        );
                        HttpResponse::Ok().json(data)
                    }
                    _ => HttpResponse::Ok().body(
                        "Your position does not allow you to create a unit in the curriculum",
                    ),
                }
            }
            None => HttpResponse::Ok().body("No owner's data"),
        }
    } else {
        HttpResponse::Ok().body("Registration unsuccessfully, you are not allowed to add the unit")
    }
}

#[get("/curriculum/all_units")]
pub async fn get_all_units() -> impl Responder {
    let all_units = get_units();
    HttpResponse::Ok().json(all_units)
}

/// This api deletes the curriculum.
/// takes the owner's id and reg number
/// checks to see if they are allowed to the system.
/// checks to see if the owner's role is allowed to the system.
/// if true, the data is deleted
#[delete("/curriculum/delete_unit")]
pub async fn delete_some_units(data: Json<DeleteUnitId>) -> impl Responder {
    let is_allowed = check_for_authority_curriculum(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_s_data = read_one_authority_curriculum(data.owner_id.clone().to_uppercase());
        match owner_s_data {
            Some(return_owner_s_data) => {
                let owner_s_role = return_owner_s_data.owner_role.clone();
                let search_owner_s_role = owner_s_role.as_str();
                match search_owner_s_role {
                    "COD" => {
                        let deleted_data = delete_units(data.course_id.clone().to_uppercase());
                        HttpResponse::Ok().body(format!("num deleted: {}", deleted_data))
                    }
                    _ => HttpResponse::Ok()
                        .body("Position not allowed to delete a unit in curriculum"),
                }
            }
            None => HttpResponse::Ok().body("There is no data of the owner"),
        }
    } else {
        HttpResponse::Ok().body("You are not allowed to use the system")
    }
}

/// patch api that updates the unit's id, name, electives or course name
/// of the unit.
/// takes the owner's id and password.
/// checks to see if the user is valuable
/// checks the role of the owner
/// if owner's role is allowed, the update is done.
#[patch("/curriculum/update_units")]
pub async fn update_some_units(units: Json<Update>) -> impl Responder {
    let data = Update {
        owner_id: units.owner_id.clone().to_uppercase(),
        owner_password: units.owner_password.clone().to_uppercase(),
        unit_identity: units.unit_identity.clone().to_uppercase(),
        current_updated_value: units.current_updated_value.clone().to_uppercase(),
        field: units.field.clone().to_uppercase(),
    };
    let is_allowed = check_for_authority_curriculum(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_s_data = read_one_authority_curriculum(data.owner_id.clone().to_uppercase());
        match owner_s_data {
            Some(return_owner_s_data) => {
                let role_of_owner = return_owner_s_data.owner_role.clone();
                let search_role_of_owner = role_of_owner.as_str();
                match search_role_of_owner {
                    "COD" => {
                        let result = update_units(
                            data.unit_identity,
                            data.current_updated_value,
                            data.field.as_str(),
                        );

                        HttpResponse::Ok().json(result)
                    }
                    _ => HttpResponse::Ok().body("your role does not allow you to the system"),
                }
            }
            None => HttpResponse::Ok().body("no owner's record found"),
        }
    } else {
        HttpResponse::Ok().body("You are not allowed ")
    }
}

#[post("/course_duration/create_course_duration_authority")]
pub async fn create_course_duration_authority(
    data: Json<CreateCourseDurationAuthority>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let data_result = create_allowed_users(
                            data.user_id.to_uppercase().clone(),
                            data.user_password.clone(),
                        );
                        match data_result {
                            Ok(created_data) => HttpResponse::Ok().json(created_data),
                            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                        }
                    }
                    _ => HttpResponse::Ok().body(
                        "Your position does not allow you go view all allowed authority data",
                    ),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Not allowed to get all authority data")
    }
}

#[get("/course_duration/get_all_allowed_authority")]
pub async fn get_all_allowed_course_duration_authority(
    data: Json<AllowedCourseDuration>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let all_data = get_all_allowed_user();
                        HttpResponse::Ok().json(all_data)
                    }
                    _ => HttpResponse::Ok().body(
                        "Your position does not allow you go view all allowed authority data",
                    ),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Not allowed to get all authority data")
    }
}

#[get("/course_duration/get_one_authority_course_duration")]
pub async fn get_one_authority_course_duration(data: Json<CourseDurationGetOne>) -> impl Responder {
    let is_allowed = check_authorisation_for_allowed_user(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone().to_uppercase(),
    );
    if is_allowed {
        let data_access = get_allowed_user(data.user_id.to_uppercase().clone());
        HttpResponse::Ok().json(data_access)
    } else {
        HttpResponse::Ok().body("Password incorrect")
    }
}

#[patch("/course_duration/update_course_duration_authority")]
pub async fn update_allowed_course_duration_authority(
    data: Json<UpdateAllowedCourseDurationCurriculum>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let staff_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match staff_data_result {
            Ok(staff_data) => {
                let staff_role = staff_data.user_role.as_str();
                match staff_role {
                    "COD" => {
                        let update_data = update_allowed_user(
                            data.user_id.to_uppercase().clone(),
                            data.new_value.clone(),
                            data.field.clone().to_uppercase(),
                        );
                        match update_data {
                            Some(Ok(return_update_data)) => {
                                HttpResponse::Ok().json(return_update_data)
                            }
                            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
                            None => HttpResponse::Ok().body("None"),
                        }
                    }
                    _ => HttpResponse::Ok().body("staff role is not allowed to use the system"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("password is incorrect")
    }
}

#[delete("/course_duration/delete_course_duration_authority")]
pub async fn delete_allowed_course_duration_authority(
    data: Json<DeleteCourseDurationAuthority>,
) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let role_owner = owner_data.user_role.as_str();
                match role_owner {
                    "COD" => {
                        let deleted_id = data.user_id.clone().to_uppercase();
                        let deleted_result = delete_allowed_user(deleted_id);
                        HttpResponse::Ok().json(deleted_result)
                    }
                    _ => HttpResponse::Ok().body("Owner's role is not allowed to the system"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("incorrect user name or password")
    }
}

#[post("/course_duration/create_course_length")]
pub async fn create_length_course(data: Json<CreateCourseDuration>) -> impl Responder {
    let is_allowed = check_authorisation_for_allowed_user(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let course_created = create_course_in_course_duration_table(
            data.course_name.clone().to_uppercase(),
            data.course_len.clone(),
        );
        HttpResponse::Ok().json(course_created)
    } else {
        HttpResponse::Ok().body("Not allowed to access the system")
    }
}

#[get("/course_duration/get_all_course_duration")]
pub async fn get_all_course_duration_data() -> impl Responder {
    let all_course_duration = get_all_courses_in_course_duration_table();
    HttpResponse::Ok().json(all_course_duration)
}

#[get("/course_duration/get_one_course_duration")]
pub async fn get_one_course_duration_data(data: Json<GetOneCourseDuration>) -> impl Responder {
    let result =
        get_course_in_course_duration_table(data.course_name.clone().to_uppercase()).unwrap();
    HttpResponse::Ok().json(result)
}

#[patch("/course_duration/update_course_duration")]
pub async fn update_course_duration_data(data: Json<UpdateCourseDuration>) -> impl Responder {
    let is_allowed = check_authorisation_for_allowed_user(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let result = update_course_in_course_duration_table(
            data.course_name.clone().to_uppercase(),
            data.new_value.clone(),
            data.field.clone().to_uppercase(),
        );
        match result {
            Some(result_data) => HttpResponse::Ok().json(result_data),
            None => HttpResponse::Ok().body("failed to update the course_duration"),
        }
    } else {
        HttpResponse::Ok().body("Not allowed to access the system")
    }
}

#[delete("/course_duration/delete_course_duration")]
pub async fn delete_course_duration_data(data: Json<DeleteCourseDuration>) -> impl Responder {
    let is_allowed = check_authorisation_for_allowed_user(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let result =
            delete_a_course_in_course_duration_table(data.course_name.clone().to_uppercase());
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Ok().body("Not allowed to access the system")
    }
}

#[post("/course_code/add_course_code")]
pub async fn add_course_code(data: Json<CourseCodeTakeIn>) -> impl Responder {
    let is_allowed = check_course_code_authority_password(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let new_course_data = create_course_code(
            data.course_name.to_uppercase().clone(),
            data.course_code.to_uppercase().clone(),
        );
        HttpResponse::Ok().json(new_course_data)
    } else {
        HttpResponse::Ok().body("Not allowed to update the course")
    }
}

#[get("/course_code/get_one_course_code")]
pub async fn course_code_one(data: Json<CourseCodeReadOne>) -> impl Responder {
    let one_course_data = read_one_course_code(data.course_name.clone().to_uppercase());
    HttpResponse::Ok().json(one_course_data)
}

#[get("/course/get_all_course_code")]
pub async fn course_code_all() -> impl Responder {
    let all_data = read_all_course_codes();
    HttpResponse::Ok().json(all_data)
}

#[patch("/course/course_code_update")]
pub async fn course_code_update(data: Json<CourseCodeUpdate>) -> impl Responder {
    let is_allowed = check_course_code_authority_password(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let updated_course_data = update_course_code(
            data.course_name.clone().to_uppercase(),
            data.field.clone().to_uppercase(),
            data.new_value.clone(),
        );
        match updated_course_data {
            Some(updated_value) => HttpResponse::Ok().json(updated_value),
            None => HttpResponse::Ok().body("Failed to update the course code"),
        }
    } else {
        HttpResponse::Ok().body("User not allowed to use the system")
    }
}

#[delete("/course/delete_course_code")]
pub async fn course_code_delete(data: Json<CourseCodeDelete>) -> impl Responder {
    let is_allowed = check_course_code_authority_password(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let deleted_data = delete_course_code(data.course_name.clone().to_uppercase());
        HttpResponse::Ok().json(deleted_data)
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[post("/course/create_course_code_authority")]
pub async fn course_code_authority_create(data: Json<CourseCodeAuthorityCreate>) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role_value = owner_data.user_role.as_str();
                match owner_role_value {
                    "COD" => {
                        let data_athority = CourseCodeAuthority {
                            user_id: data.user_id.clone().to_uppercase(),
                            user_password: data.user_password.clone(),
                            user_first_name: data.user_first_name.clone().to_uppercase(),
                            user_middle_name: data.user_middle_name.to_uppercase().clone(),
                            user_last_name: data.user_last_name.to_uppercase().clone(),
                            user_role: data.user_role.to_uppercase().clone(),
                        };
                        let result = create_course_code_authority(data_athority);
                        // let return_data = CourseCodeAuthorityReturn {
                        //     user_id: result.user_id,
                        //     user_first_name: result.user_first_name,
                        //     user_middle_name: result.user_middle_name,
                        //     user_last_name: result.user_last_name,
                        //     user_role: result.user_role,
                        // };
                        HttpResponse::Ok().json(result)
                    }
                    _ => HttpResponse::Ok()
                        .body("Owner's role does not allow  him to create course_code authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[get("/course/course_code_authority_read_one")]
pub async fn course_code_authority_read_one(
    data: Json<CourseCodeAuthorityReadOne>,
) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role_value = owner_data.user_role.as_str();
                match owner_role_value {
                    "COD" => {
                        let user_data =
                            read_one_course_code_authority(data.user_id.to_uppercase().clone());
                        // let return_user_data = CourseCodeAuthorityReturn {
                        //     user_id: user_data.user_id,
                        //     user_first_name: user_data.user_first_name,
                        //     user_middle_name: user_data.user_middle_name,
                        //     user_last_name: user_data.user_last_name,
                        //     user_role: user_data.user_role,
                        // };
                        HttpResponse::Ok().json(user_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("Owner's role does not allow  him to create course_code authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[get("/course/course_code_authority_read_all")]
pub async fn course_code_authority_read_all(
    data: Json<CourseCodeAuthorityReadAll>,
) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role_value = owner_data.user_role.as_str();
                match owner_role_value {
                    "COD" => {
                        let user_data = read_all_course_code_authority();

                        HttpResponse::Ok().json(user_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("Owner's role does not allow  him to create course_code authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[patch("/course/course_code_authority_update")]
pub async fn course_code_authority_update(data: Json<CourseCodeAuthorityUpdate>) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role_value = owner_data.user_role.as_str();
                match owner_role_value {
                    "COD" => {
                        let returning_update_data = update_course_code_authority(
                            data.user_id.to_uppercase().clone(),
                            data.field.to_uppercase().clone(),
                            data.new_value.clone(),
                        );
                        match returning_update_data {
                            Some(output_data) => HttpResponse::Ok().json(output_data),
                            None => HttpResponse::Ok().body("Failed to update the data"),
                        }
                    }
                    _ => HttpResponse::Ok()
                        .body("Owner's role does not allow  him to create course_code authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[delete("/course/course_code_authority_delete")]
pub async fn course_code_authority_delete(data: Json<CourseCodeAuthorityDelete>) -> impl Responder {
    let is_logged = check_for_staff_authorization(
        data.owner_id.clone().to_uppercase(),
        data.owner_password.clone(),
    );
    if is_logged {
        let owner_data_result = read_one_staff(data.owner_id.clone().to_uppercase());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role_value = owner_data.user_role.as_str();
                match owner_role_value {
                    "COD" => {
                        let deleted_data =
                            delete_course_code_authority(data.user_id.to_uppercase().clone());
                        HttpResponse::Ok().json(deleted_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("Owner's role does not allow  him to create course_code authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("User is not allowed to use the system")
    }
}

#[post("/fees/fees_structure_create")]
pub async fn fees_structure_create(data: Json<FeeStructureCreate>) -> impl Responder {
    let is_allowed = check_password_fees_authority(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data = FeeStructure {
            course_name: data.course_name.to_uppercase().clone(),
            year: data.year,
            semester: data.semester,
            expenditure_name: data.expenditure_name.to_uppercase().clone(),
            expenditure_cost: data.expenditure_cost,
        };
        let result_created_data = create_fees_structure(created_data);
        HttpResponse::Ok().json(result_created_data)
    } else {
        HttpResponse::Ok().body("Not authorised to create a new fee structure")
    }
}

#[get("/fees/read_one_course_expenditure_cost_fees_structure")]
pub async fn read_one_course_expenditure_cost_fees_structure(
    data: Json<FeeStructureGetOne>,
) -> impl Responder {
    let return_data = read_one_course_expenditure_cost(data.course_name.to_uppercase().clone());
    HttpResponse::Ok().json(return_data)
}

#[get("/fees/read_one_year_course_expenditure_fees_structure")]
pub async fn read_one_year_course_expenditure_fees_structure(
    data: Json<FeeStructureGetOneYear>,
) -> impl Responder {
    let return_data =
        read_one_year_expenditure_cost(data.course_name.to_uppercase().clone(), data.year);
    HttpResponse::Ok().json(return_data)
}

#[get("/fees/read_one_sem_year_course_expenditure_fees_structure")]
pub async fn read_one_sem_year_course_expenditure_fees_structure(
    data: Json<FeeStructureGetOneSem>,
) -> impl Responder {
    let return_data =
        read_one_sem_expenditure_cost(data.course_name.to_uppercase().clone(), data.year, data.sem);

    HttpResponse::Ok().json(return_data)
}

#[get("/fees/read_one_expenditure_cost_fees_structure")]
pub async fn read_one_expenditure_cost_fees_structure(
    data: Json<FeeStructureGetOneExpenditure>,
) -> impl Responder {
    let return_data = read_one_expenditure_cost(
        data.course_name.to_uppercase().clone(),
        data.year,
        data.sem,
        data.expenditure_name.to_uppercase().clone(),
    );
    let cost = OneExpenditureCost { cost: return_data };
    HttpResponse::Ok().json(cost)
}

#[get("/fees/read_all_expenditure_fees_structure")]
pub async fn read_all_expenditure_fees_structure() -> impl Responder {
    let return_data = read_all_fees_authority();
    HttpResponse::Ok().json(return_data)
}

#[patch("/fees/update_fees_expenditure_cost")]
pub async fn update_fees_expenditure_cost(data: Json<FeeStructureUpdate>) -> impl Responder {
    let is_allowed = check_password_fees_authority(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let return_data = update_ependiture_fees_structure(
            data.name_course.to_uppercase().clone(),
            data.course_year,
            data.course_sem,
            data.name_expenditure.clone().to_uppercase(),
            data.field.clone().to_uppercase(),
            data.new_value.clone(),
        );
        match return_data {
            Some(updated_data) => HttpResponse::Ok().json(updated_data),
            None => HttpResponse::Ok()
                .body("Failed to update the fees expenditure in fees structure table"),
        }
    } else {
        HttpResponse::Ok().json("The owner is not allowed to delete the expenditure")
    }
}

#[delete("/fees/delete_expenditure_fees_structure")]
pub async fn delete_expenditure_fees_structure(data: Json<FeeStructureDelete>) -> impl Responder {
    let is_allowed = check_password_fees_authority(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let deleted_data = delete_expenditure_fee_structure(
            data.name_course.clone().to_uppercase(),
            data.course_year,
            data.course_sem,
            data.name_expenditure.clone().to_uppercase(),
        );
        HttpResponse::Ok().json(deleted_data)
    } else {
        HttpResponse::Ok().json("The owner is not allowed to delete the expenditure")
    }
}

#[post("/fees/fees_authority_create")]
pub async fn fees_authority_create(data: Json<FeesAuthorityCreate>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let data_create = FeesAuthority {
                            user_id: data.user_id.to_uppercase().clone(),
                            user_password: data.user_password.to_uppercase().clone(),
                            user_first_name: data.user_first_name.to_uppercase().clone(),
                            user_middle_name: data.user_middle_name.to_uppercase().clone(),
                            user_last_name: data.user_last_name.to_uppercase().clone(),
                            user_role: data.user_role.to_uppercase().clone(),
                        };
                        let created_data = create_fees_authority(data_create);
                        HttpResponse::Ok().json(created_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("The owner's role does not allow you to create authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().json("User is not allowed to create the fees authority table")
    }
}

#[get("/fees/fees_authority_read_one")]
pub async fn fees_authority_read_one(data: Json<FeesAuthorityReadOne>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let result_data =
                            read_one_fees_authority(data.user_id.to_uppercase().clone());
                        HttpResponse::Ok().json(result_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("The owner's role does not allow you to create authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().json("User is not allowed to read the fees authority table")
    }
}

#[get("/fees/fees_authority_read_all")]
pub async fn fees_authority_read_all(data: Json<FeesAuthorityReadAll>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let result_data = read_all_fees_authority();
                        HttpResponse::Ok().json(result_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("The owner's role does not allow you to create authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().json("User is not allowed to read all data in fees authority table")
    }
}

#[patch("/fees/fees_authority_update")]
pub async fn fees_authority_update(data: Json<FeesAuthorityUpdate>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let result_data = update_fees_authority(
                            data.user_identity.to_uppercase().clone(),
                            data.field.to_uppercase().clone(),
                            data.new_value.to_uppercase().clone(),
                        );
                        match result_data {
                            Some(fees_authority_data) => {
                                HttpResponse::Ok().json(fees_authority_data)
                            }
                            None => {
                                HttpResponse::Ok().body("Failed to update the data in the database")
                            }
                        }
                    }
                    _ => HttpResponse::Ok()
                        .body("The owner's role does not allow you to create authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().json("User is not allowed to update fees authority table")
    }
}

#[delete("/fees/fees_authority_delete")]
pub async fn fees_authority_delete(data: Json<FeesAuthorityDelete>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                let owner_role = owner_data.user_role.as_str();
                match owner_role {
                    "COD" => {
                        let deleted_data =
                            delete_fees_authority(data.user_id.to_uppercase().clone());
                        HttpResponse::Ok().json(deleted_data)
                    }
                    _ => HttpResponse::Ok()
                        .body("The owner's role does not allow you to create authority"),
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().json("User is not allowed to delete data in fees authority table")
    }
}

#[post("/graduation/create_authority_graduation")]
pub async fn create_graduation_authority(data: Json<CreateGraduationAuthority>) -> impl Responder {
    let is_logged_in = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_logged_in {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data = AuthorityGraduation {
                        user_id: data.user_id.to_uppercase().clone(),
                        user_password: data.user_password.clone(),
                        user_role: data.user_role.to_uppercase().clone(),
                        user_first_name: data.user_first_name.to_uppercase().clone(),
                        user_middle_name: data.user_middle_name.to_uppercase().clone(),
                        user_last_name: data.user_last_name.to_uppercase().clone(),
                    };
                    let user_returned_data = create_authority_graduation(created_data);
                    match user_returned_data {
                        Ok(new_data) => HttpResponse::Ok().json(new_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok().body("The owner's role does not allow him or her to insert authority graduation data into the database.")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[get("/graduation/get_one_authority_graduation")]
pub async fn get_one_authority_graduation(data: Json<GetOneGraduationAuthority>) -> impl Responder {
    let is_logged_in = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_logged_in {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let user_returned_data =
                        read_one_authority_graduation(data.user_id.to_uppercase().clone());
                    match user_returned_data {
                        Ok(new_data) => HttpResponse::Ok().json(new_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok().body("The owner's role does not allow him or her to insert authority graduation data into the database.")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[get("/graduation/get_all_authority_graduation")]
pub async fn get_all_authority_graduation(data: Json<GetAllGraduationAuthority>) -> impl Responder {
    let is_logged_in = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_logged_in {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let user_returned_data = read_all_authority_graduation();
                    match user_returned_data {
                        Ok(new_data) => HttpResponse::Ok().json(new_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok().body("The owner's role does not allow him or her to insert authority graduation data into the database.")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[patch("/graduation/update_authority_graduation")]
pub async fn update_graduation_authority(data: Json<UpdateGraduationAuthority>) -> impl Responder {
    let is_logged_in = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_logged_in {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let user_returned_data = update_authority_graduation(
                        data.user_id.to_uppercase().clone(),
                        data.field.to_uppercase().clone(),
                        data.new_value.clone(),
                    );
                    match user_returned_data {
                        Some(Ok(new_data)) => HttpResponse::Ok().json(new_data),
                        Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
                        None => HttpResponse::Ok()
                            .body("Please enter a valid field to update in the database"),
                    }
                } else {
                    HttpResponse::Ok().body("The owner's role does not allow him or her to insert authority graduation data into the database.")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[delete("/graduation/delete_graduation_authority")]
pub async fn delete_graduation_authority(data: Json<DeleteGraduationAuthority>) -> impl Responder {
    let is_logged_in = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_logged_in {
        let owner_data_result = read_one_staff(data.owner_id.to_uppercase().clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let user_returned_data =
                        delete_authority_graduation(data.user_id.to_uppercase().clone());
                    match user_returned_data {
                        Ok(new_data) => HttpResponse::Ok().json(new_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok().body("The owner's role does not allow him or her to insert authority graduation data into the database.")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[post("/graduation/graduation_student_create")]
pub async fn graduation_student_create(data: Json<GraduationStudentCreate>) -> impl Responder {
    let is_logged_in = check_authority_graduation_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    let month: i32 = current_month()
        .try_into()
        .expect("Failed to convert the current month from i32 to u32");
    let day: i32 = current_day()
        .try_into()
        .expect("Failed to convert u32 to i32 value");

    if is_logged_in {
        let created_data = StudentsGraduation {
            student_reg_no: data.student_reg_no.to_uppercase().clone(),
            student_first_name: data.student_first_name.to_uppercase().clone(),
            student_middle_name: data.student_middle_name.to_uppercase().clone(),
            student_last_name: data.student_last_name.to_uppercase().clone(),
            student_course_year: data.student_course_year,
            student_course: data.student_course.to_uppercase().clone(),
            student_programme: data.student_programme.to_uppercase().clone(),
            student_department: data.student_department.to_uppercase().clone(),
            student_school: data.student_school.to_uppercase().clone(),
            student_class: data.student_class.to_uppercase().clone(),
            student_gssp: data.student_gssp.to_uppercase().clone(),
            student_gender: data.student_gender.to_uppercase().clone(),
            student_role: data.student_role.clone(),
            student_hostel: data.student_hostel.clone(),
            graduation_year: current_year(),
            graduation_month: month,
            graduation_day: day,
        };
        let created_returned_data = create_students_graduation(created_data);
        match created_returned_data {
            Ok(new_created_data) => HttpResponse::Ok().json(new_created_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[get("/graduation/graduation_student_read_one")]
pub async fn graduation_student_read_one(data: Json<GraduationStudentReadOne>) -> impl Responder {
    let is_logged_in = check_authority_graduation_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );

    if is_logged_in {
        let one_students_graduation_returned_data =
            read_one_students_graduation(data.student_reg_no.to_uppercase().clone());
        match one_students_graduation_returned_data {
            Ok(students_data) => HttpResponse::Ok().json(students_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}
#[get("/graduation/graduation_student_read_all")]
pub async fn graduation_student_read_all(data: Json<GraduationStudentReadAll>) -> impl Responder {
    let is_logged_in = check_authority_graduation_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );

    if is_logged_in {
        let all_data_result = read_all_students_graduation();
        match all_data_result {
            Ok(students_data) => HttpResponse::Ok().json(students_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[delete("/graduation/graduation_student_delete")]
pub async fn graduation_student_delete(data: Json<GraduationStudentReadOne>) -> impl Responder {
    let is_logged_in = check_authority_graduation_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );

    if is_logged_in {
        let all_data_result =
            delete_students_graduation(data.student_reg_no.to_uppercase().clone());
        match all_data_result {
            Ok(students_data) => HttpResponse::Ok().json(students_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's username or password")
    }
}

#[post("/hostels/hostels_authority_create")]
pub async fn hostels_authority_create(data: Json<HostelsAuthorityCreate>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data = HostelsAuthority {
            user_id: data.user_id.to_uppercase().clone(),
            user_password: data.user_password.clone(),
            user_first_name: data.user_first_name.to_uppercase().clone(),
            user_middle_name: data.user_middle_name.to_uppercase().clone(),
            user_last_name: data.user_last_name.to_uppercase().clone(),
            user_role: data.user_role.to_uppercase().clone(),
        };
        let created_result = create_hostels_authority(created_data);
        match created_result {
            Ok(created_data) => HttpResponse::Ok().json(created_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/hostels/hostels_authority_read_one")]
pub async fn hostels_authority_read_one(data: Json<HostelsAuthorityReadOne>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let user_data_result = read_one_hostel_authority(data.user_id.to_uppercase().clone());
        match user_data_result {
            Ok(user_data) => HttpResponse::Ok().json(user_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/hostels/hostels_authority_read_all")]
pub async fn hostels_authority_read_all(data: Json<HostelsAuthorityReadAll>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let user_data_result = read_all_hostel_authority();
        match user_data_result {
            Ok(user_data) => HttpResponse::Ok().json(user_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[patch("/hostels/hostels_authority_update")]
pub async fn hostels_authority_update(data: Json<HostelsAuthorityUpdate>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let user_data_result = update_hostels_authority(
            data.user_id.to_uppercase().clone(),
            data.field.to_uppercase().clone(),
            data.new_value.clone(),
        );
        match user_data_result {
            Some(Ok(user_data)) => HttpResponse::Ok().json(user_data),
            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
            None => HttpResponse::Ok().body("Enter a valid field"),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[delete("/hostels/hostels_authority_delete")]
pub async fn hostels_authority_delete(data: Json<HostelsAuthorityReadOne>) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let user_data_result = delete_hostels_authority(data.user_id.to_uppercase().clone());
        match user_data_result {
            Ok(user_data) => HttpResponse::Ok().json(user_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[post("/hostels/student_hostels_create")]
pub async fn student_hostels_create(data: Json<StudentsHostelsCreate>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data = StudentHostel {
            student_reg_no: data.student_reg_no.to_uppercase().clone(),
            student_first_name: data.student_first_name.to_uppercase().clone(),
            student_middle_name: data.student_middle_name.to_uppercase().clone(),
            student_last_name: data.student_last_name.to_uppercase().clone(),
            hostel_name: data.hostel_name.to_uppercase().clone(),
            hostel_room_number: data.hostel_room_number,
            gender: data.gender.to_uppercase().clone(),
        };
        let returned_created_result = create_student_hostel(created_data);
        match returned_created_result {
            Ok(returned_created_data) => HttpResponse::Ok().json(returned_created_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/hostels/student_hostels_read_one")]
pub async fn student_hostels_read_one(data: Json<StudentsHostelsCreate>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let returned_created_result =
            read_one_student_hostel(data.student_reg_no.to_uppercase().clone());
        match returned_created_result {
            Ok(returned_created_data) => HttpResponse::Ok().json(returned_created_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/hostels/student_hostels_read_all")]
pub async fn student_hostels_read_all(data: Json<StudentsHostelsCreate>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let returned_created_result =
            read_one_student_hostel(data.student_reg_no.to_uppercase().clone());
        match returned_created_result {
            Ok(returned_created_data) => HttpResponse::Ok().json(returned_created_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[patch("/hostels/student_hostels_update")]
pub async fn student_hostels_update(data: Json<StudentsHostelsUpdate>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let returned_created_result = update_student_hostel(
            data.user_id.to_uppercase().clone(),
            data.field.to_uppercase().clone(),
            data.new_value.to_uppercase().clone(),
        );
        match returned_created_result {
            Some(Ok(returned_created_data)) => HttpResponse::Ok().json(returned_created_data),
            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
            None => HttpResponse::Ok().body("Enter a valid field"),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[delete("/hostels/student_hostels_delete")]
pub async fn student_hostels_delete(data: Json<StudentsHostelsReadOne>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let returned_created_result =
            delete_student_hostel(data.student_reg_no.to_uppercase().clone());
        match returned_created_result {
            Ok(returned_created_data) => HttpResponse::Ok().json(returned_created_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[post("/hostels/hostels_create")]
pub async fn hostels_create(data: Json<HostelsCreate>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let input_data = Hostels {
                        hostel_name: data.hostel_name.to_uppercase().clone(),
                        no_of_rooms: data.no_of_rooms,
                        students_per_room: data.students_per_room,
                        gender: data.gender.to_uppercase().clone(),
                    };
                    let created_data_result = create_hostel(input_data);
                    match created_data_result {
                        Ok(output_data) => HttpResponse::Ok().json(output_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/hostels/hostels_get_one")]
pub async fn hostels_get_one(data: Json<HostelsGetOne>) -> impl Responder {
    let created_data_result = read_one_hostel(data.hostel_name.to_uppercase().clone());
    match created_data_result {
        Ok(output_data) => HttpResponse::Ok().json(output_data),
        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
    }
}

#[get("/hostels/hostels_get_all")]
pub async fn hostels_get_all() -> impl Responder {
    let created_data_result = read_all_hostels();
    match created_data_result {
        Ok(output_data) => HttpResponse::Ok().json(output_data),
        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
    }
}

#[patch("/hostels/hostels_update")]
pub async fn hostels_update(data: Json<HostelsUpdate>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data_result = update_hostel(
                        data.hostel_name.to_uppercase().clone(),
                        data.field.to_uppercase().clone(),
                        data.new_value.clone(),
                    );
                    match created_data_result {
                        Some(Ok(output_data)) => HttpResponse::Ok().json(output_data),
                        Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
                        None => HttpResponse::Ok().body("Enter a valid field to update"),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[delete("/hostels/hostels_delete")]
pub async fn hostels_delete(data: Json<HostelsDelete>) -> impl Responder {
    let is_allowed = check_hostel_authority_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data_result =
                        delete_hostel(data.hostel_name.to_uppercase().clone());
                    match created_data_result {
                        Ok(output_data) => HttpResponse::Ok().json(output_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[post("/graduation/course_graduation_authority_create")]
pub async fn course_graduation_authority_create(
    data: Json<CourseGraduationAuthorityCreate>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let input_data = CourseGraduationAuthority {
                        user_id: data.user_id.to_uppercase().clone(),
                        user_password: data.user_password.clone(),
                        user_first_name: data.user_first_name.to_uppercase().clone(),
                        user_middle_name: data.user_middle_name.to_uppercase().clone(),
                        user_last_name: data.user_last_name.to_uppercase().clone(),
                        user_role: data.user_role.to_uppercase().clone(),
                    };
                    let created_data_result = create_course_graduation_authority(input_data);
                    match created_data_result {
                        Ok(output_data) => HttpResponse::Ok().json(output_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/graduation/course_graduation_authority_get_one")]
pub async fn course_graduation_authority_get_one(
    data: Json<CourseGraduationAuthorityGetOne>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data_result =
                        read_one_course_graduation_authority(data.user_id.to_uppercase().clone());
                    match created_data_result {
                        Ok(output_data) => {
                            let no_passkey_output = CourseGraduationAuthorityReturn {
                                user_id: output_data.user_id,
                                user_first_name: output_data.user_first_name,
                                user_middle_name: output_data.user_middle_name,
                                user_last_name: output_data.user_last_name,
                                user_role: output_data.user_role,
                            };
                            HttpResponse::Ok().json(no_passkey_output)
                        }
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/graduation/course_graduation_authority_get_all")]
pub async fn course_graduation_authority_get_all(
    data: Json<CourseGraduationAuthorityGetAll>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data_result = read_all_course_graduation_authority();
                    match created_data_result {
                        Ok(output_data) => HttpResponse::Ok().json(output_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[patch("/graduation/course_graduation_authority_update")]
pub async fn course_graduation_authority_update(
    data: Json<CourseGraduationAuthorityUpdate>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data_result = updated_course_graduation_authority(
                        data.user_id.to_uppercase().clone(),
                        data.field.to_uppercase().clone(),
                        data.new_value.clone(),
                    );
                    match created_data_result {
                        Some(Ok(output_data)) => HttpResponse::Ok().json(output_data),
                        Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
                        None => {
                            HttpResponse::Ok().body("Please pass in a valid column field to update")
                        }
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[delete("/graduation/course_graduation_authority_delete")]
pub async fn course_graduation_authority_delete(
    data: Json<CourseGraduationAuthorityGetOne>,
) -> impl Responder {
    let is_allowed = check_for_staff_authorization(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let owner_data_result = read_one_staff(data.owner_id.clone());
        match owner_data_result {
            Ok(owner_data) => {
                if owner_data.user_role.as_str() == "COD" {
                    let created_data_result =
                        delete_course_graduation_authority(data.user_id.to_uppercase().clone());
                    match created_data_result {
                        Ok(output_data) => HttpResponse::Ok().json(output_data),
                        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
                    }
                } else {
                    HttpResponse::Ok()
                        .body("The owner's role does not allow it to create the hostel")
                }
            }
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[post("/graduation/total_graduation_create")]
pub async fn total_graduation_create(data: Json<TotalGraduationCreate>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let input_data = TotalGraduation {
            graduation_year: data.graduation_year,
            number_students: data.number_students,
            first_class_students: data.first_class_students,
            second_class_upper_division_students: data.second_class_upper_division_students,
            second_class_lower_division_students: data.second_class_lower_division_students,
            pass: data.pass,
            fail: data.fail,
        };
        let created_data_result = create_total_graduation(input_data);
        match created_data_result {
            Ok(output_data) => HttpResponse::Ok().json(output_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/graduation/total_graduation_get_one")]
pub async fn total_graduation_get_one(data: Json<TotalGraduationGetOne>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data_result = read_one_total_graduation_data(data.graduation_year);
        match created_data_result {
            Ok(output_data) => HttpResponse::Ok().json(output_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/graduation/total_graduation_get_all")]
pub async fn total_graduation_get_all(data: Json<TotalGraduationGetAll>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data_result = read_all_total_graduation_data();
        match created_data_result {
            Ok(output_data) => HttpResponse::Ok().json(output_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[patch("/graduation/total_graduation_update")]
pub async fn total_graduation_update(data: Json<TotalGraduationUpdate>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data_result = update_total_graduation(
            data.graduation_year,
            data.field.to_uppercase().clone(),
            data.new_value,
        );
        match created_data_result {
            Some(Ok(output_data)) => HttpResponse::Ok().json(output_data),
            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
            None => HttpResponse::Ok()
                .body("Enter a valid field to udpate the total_graduation database."),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[delete("/graduation/total_graduation_delete")]
pub async fn total_graduation_delete(data: Json<TotalGraduationGetOne>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data_result = delete_total_graduation(data.graduation_year);
        match created_data_result {
            Ok(output_data) => HttpResponse::Ok().json(output_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[post("/graduation/course_graduation_create")]
pub async fn course_graduation_create(data: Json<CourseGraduationCreate>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let input_data = CourseGraduation {
            graduation_year: data.graduation_year,
            course_name: data.course_name.to_uppercase().clone(),
            number_students: data.number_students,
            first_class_students: data.first_class_students,
            second_class_upper_division_students: data.second_class_upper_division_students,
            second_class_lower_division_students: data.second_class_lower_division_students,
            pass: data.pass,
            fail: data.fail,
            department: data.department.to_uppercase().clone(),
            programme: data.programme.to_uppercase().clone(),
        };
        let created_data_result = create_course_graduation(input_data);
        match created_data_result {
            Ok(output_data) => HttpResponse::Ok().json(output_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[get("/graduation/course_graduation_get_one")]
pub async fn course_graduation_get_one(data: Json<CourseGraduationGetOne>) -> impl Responder {
    let created_data_result = read_one_course_graduation(
        data.course_name.to_uppercase().clone(),
        data.graduation_year,
    );
    match created_data_result {
        Ok(output_data) => HttpResponse::Ok().json(output_data),
        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
    }
}

#[get("/graduation/course_graduation_get_all")]
pub async fn course_graduation_get_all() -> impl Responder {
    let created_data_result = read_all_course_graduation();
    match created_data_result {
        Ok(output_data) => HttpResponse::Ok().json(output_data),
        Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
    }
}

#[patch("/graduation/course_graduation_update")]
pub async fn course_graduation_update(data: Json<CourseGraduationUpdate>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data_result = update_course_graduation(
            data.course_name.to_uppercase().clone(),
            data.field.to_uppercase().clone(),
            data.new_value.clone(),
        );
        match created_data_result {
            Some(Ok(output_data)) => HttpResponse::Ok().json(output_data),
            Some(Err(e)) => HttpResponse::Ok().body(format!("{e:?}")),
            None => HttpResponse::Ok().body("Please enter a valid field to update."),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[delete("/graduation/course_graduation_delete")]
pub async fn course_graduation_delete(data: Json<CourseGraduationDelete>) -> impl Responder {
    let is_allowed = check_course_graduation_authority_passkey(
        data.owner_id.to_uppercase().clone(),
        data.owner_password.clone(),
    );
    if is_allowed {
        let created_data_result = delete_course_graduation(
            data.graduation_year,
            data.course_name.to_uppercase().clone(),
        );
        match created_data_result {
            Ok(output_data) => HttpResponse::Ok().json(output_data),
            Err(e) => HttpResponse::Ok().body(format!("{e:?}")),
        }
    } else {
        HttpResponse::Ok().body("Incorrect owner's name or password")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = get_all_first_year_init_authority();
    println!("{:?}", data);
    get_time_now();
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(login_student)
            .service(sign_up_student)
            .service(get_students_data_all)
            .service(get_one_students_data)
            .service(remove_students_data_all)
            .service(sign_up_staff)
            .service(get_all_authority_first_year_init)
            .service(staff_delete)
            .service(staff_read_all)
            .service(staff_read_one)
            .service(staff_update)
            .service(read_authority_curriculum_all)
            .service(read_authority_curriculum_one)
            .service(updates_curriculum_authority)
            .service(all_units_reg)
            .service(get_all_units)
            .service(delete_some_units)
            .service(update_some_units)
            .service(create_course_duration_authority)
            .service(get_all_allowed_course_duration_authority)
            .service(update_allowed_course_duration_authority)
            .service(get_one_authority_course_duration)
            .service(delete_allowed_course_duration_authority)
            .service(create_length_course)
            .service(get_all_course_duration_data)
            .service(get_one_course_duration_data)
            .service(update_course_duration_data)
            .service(delete_course_duration_data)
            .service(add_course_code)
            .service(course_code_one)
            .service(course_code_all)
            .service(course_code_update)
            .service(course_code_delete)
            .service(course_code_authority_create)
            .service(course_code_authority_read_all)
            .service(course_code_authority_read_one)
            .service(course_code_authority_update)
            .service(course_code_authority_delete)
            .service(fees_structure_create)
            .service(read_one_course_expenditure_cost_fees_structure)
            .service(read_one_year_course_expenditure_fees_structure)
            .service(read_one_sem_year_course_expenditure_fees_structure)
            .service(read_one_expenditure_cost_fees_structure)
            .service(read_all_expenditure_fees_structure)
            .service(update_fees_expenditure_cost)
            .service(delete_expenditure_fees_structure)
            .service(fees_authority_create)
            .service(fees_authority_read_one)
            .service(fees_authority_read_all)
            .service(fees_authority_update)
            .service(fees_authority_delete)
            .service(create_graduation_authority)
            .service(get_one_authority_graduation)
            .service(get_all_authority_graduation)
            .service(update_graduation_authority)
            .service(delete_graduation_authority)
            .service(graduation_student_create)
            .service(graduation_student_read_one)
            .service(graduation_student_read_all)
            .service(graduation_student_delete)
            .service(hostels_authority_create)
            .service(hostels_authority_read_one)
            .service(hostels_authority_read_all)
            .service(hostels_authority_update)
            .service(hostels_authority_delete)
            .service(student_hostels_create)
            .service(student_hostels_read_one)
            .service(student_hostels_read_all)
            .service(student_hostels_update)
            .service(student_hostels_delete)
            .service(hostels_create)
            .service(hostels_get_one)
            .service(hostels_get_all)
            .service(hostels_update)
            .service(hostels_delete)
            .service(course_graduation_authority_create)
            .service(course_graduation_authority_get_one)
            .service(course_graduation_authority_get_all)
            .service(course_graduation_authority_update)
            .service(course_graduation_authority_delete)
            .service(total_graduation_create)
            .service(total_graduation_get_one)
            .service(total_graduation_get_all)
            .service(total_graduation_update)
            .service(total_graduation_delete)
            .service(course_graduation_create)
            .service(course_graduation_get_one)
            .service(course_graduation_get_all)
            .service(course_graduation_update)
            .service(course_graduation_delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
