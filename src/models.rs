use diesel::{
    pg::{self, Pg},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::schema;

#[derive(Deserialize, Serialize)]
pub struct StudentInitAuthorityCheckUp {
    pub owner_id: String,
    pub owner_password: String,
    pub reg_no: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub programme: String,
    pub course: String,
    pub department: String,
    pub school: String,
    pub class: String,
    pub gssp: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub reg_no: String,
    pub password: String,
}

/// curriculum struct representing The curriculum database,
/// used to update the student's curriculum database
/// every unit_id has its own name, elective and course_name
#[derive(Insertable, Queryable, Selectable, PartialEq, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::curriculum)]
#[diesel(check_for_backend(Pg))]
pub struct Curriculum {
    pub unit_id: String,
    pub unit_name: String,
    pub electives: String,
    pub course_name: String,
}

/// this is the format that will be used to enter when posting a data
#[derive(Serialize, Deserialize)]
pub struct CurriculumEntrance {
    pub owner_id: String,
    pub owner_password: String,
    pub unit_id: String,
    pub unit_name: String,
    pub electives: String,
    pub course_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUnitId {
    pub owner_id: String,
    pub owner_password: String,
    pub course_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub owner_id: String,
    pub owner_password: String,
    pub unit_identity: String,
    pub current_updated_value: String,
    pub field: String,
}
/// It will be updated by the admin
#[derive(Insertable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name=schema::course_duration)]
#[diesel(check_for_backend(Pg))]
pub struct CourseDuration {
    pub course_name: String,
    pub course_length: i32,
}

#[derive(Insertable, Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = crate::schema::access_to_course_duration)]
#[diesel(check_for_backend(Pg))]
pub struct AccessToCourseDuration {
    pub id: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckCourseLength {
    pub owner_id: String,
    pub owner_password: String,
    pub id: String,
    pub course_name: String,
    pub length: i32,
    pub purpose: String,
}

#[derive(Insertable, Selectable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::first_year_init_authority)]
#[diesel(check_for_backend(pg::Pg))]
pub struct FirstYearInitAuthority {
    pub id: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub role: String,
}
/*
CREATE TABLE first_year_init_authority(
    id VARCHAR PRIMARY KEY,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    role VARCHAR NOT NULL
)
*/

#[derive(Serialize, Deserialize)]
pub struct FirstYearInitAuthorityGet {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Serialize, Deserialize)]
pub struct FirstInitGetAllAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct FirstInitDeleteAAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub role: String,
    pub user_id: String,
}

#[derive(Selectable, Insertable, Queryable, Debug, Serialize)]
#[diesel(table_name = crate::schema::curriculum_authority)]
#[diesel(check_for_backend(Pg))]
pub struct CurriculumAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub owner_role: String,
}
/*
CREATE TABLE curriculum_authority(
    owner_id VARCHAR PRIMARY KEY,
    owner_password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    owner_role VARCHAR NOT NULL
);
*/
#[derive(Serialize, Deserialize)]
pub struct CurriculumAuthorityTakeIn {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub first_name: String,
    pub last_name: String,
    pub owner_role: String,
    pub task: String,
}

#[derive(Deserialize)]
pub struct CurriculumReadAllAuthority {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct CurriculumReadOneAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct CurriculumAuthorityUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub field: String,
    pub user_identity: String,
    pub user_current_data: String,
}

#[derive(Selectable, Insertable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::staff)]
#[diesel(check_for_backend(Pg))]
pub struct Staff {
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_age: i32,
    pub department: String,
    pub user_role: String,
    pub user_position: String,
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

#[derive(Serialize, Deserialize)]
pub struct StaffUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_age: i32,
    pub department: String,
    pub user_role: String,
    pub user_position: String,
}

#[derive(Deserialize)]
pub struct StaffDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct StaffReadOne {
    pub user_id: String,
    pub user_password: String,
}

#[derive(Deserialize)]
pub struct UpdateStaff {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct StaffReadAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct StudentsDataAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct AllowedCourseDuration {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct CreateCourseDurationAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
}
#[derive(Deserialize)]
pub struct UpdateAllowedCourseDurationCurriculum {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct CourseDurationGetOne {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct DeleteCourseDurationAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}
#[derive(Deserialize)]
pub struct CreateCourseDuration {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
    pub course_len: i32,
}

#[derive(Deserialize)]
pub struct DeleteCourseDuration {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct GetOneCourseDuration {
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct UpdateCourseDuration {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Insertable, Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = crate::schema::students)]
#[diesel(check_for_backend(Pg))]
pub struct Students {
    pub reg_no: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub course: String,
    pub programme: String,
    pub department: String,
    pub school: String,
    pub class: String,
    pub gssp: String,
    pub gender: String,
    pub students_role: String,
    pub admission_date: i32,
    pub admission_month: i32,
    pub admission_year: i32,
}

#[derive(Deserialize)]
pub struct SignUpStudent {
    pub owner_id: String,
    pub owner_password: String,
    pub reg_no: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub course: String,
    pub programme: String,
    pub department: String,
    pub school: String,
    pub class: String,
    pub gssp: String,
    pub gender: String,
    pub students_role: String,
}

#[derive(Serialize)]
pub struct ReturnSignUpStudent {
    pub reg_no: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub course: String,
    pub programme: String,
    pub department: String,
    pub school: String,
    pub class: String,
    pub gssp: String,
    pub students_role: String,
    pub admission_date: i32,
    pub admission_month: i32,
    pub admission_year: i32,
}

#[derive(Deserialize)]
pub struct RemoveStudentsData {
    pub owner_id: String,
    pub owner_password: String,
    pub reg_no: String,
}

#[derive(Deserialize)]
pub struct OneStudentData {
    pub owner_id: String,
    pub owner_password: String,
    pub reg_no: String,
}

#[derive(Selectable, Queryable, Insertable, Serialize, Debug)]
#[diesel(table_name=crate::schema::fees_authority)]
#[diesel(check_for_backend(Pg))]
pub struct FeesAuthority {
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Insertable, Selectable, Queryable, Serialize)]
#[diesel(table_name=crate::schema::course_codes)]
#[diesel(check_for_backend(Pg))]
pub struct CourseCodes {
    pub course_name: String,
    pub course_code: String,
}

#[derive(Deserialize)]
pub struct CourseCodeTakeIn {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
    pub course_code: String,
}

#[derive(Deserialize)]
pub struct CourseCodeReadOne {
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct CourseCodeDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct CourseCodeUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Selectable, Insertable, Queryable, Debug, Serialize)]
#[diesel(table_name=crate::schema::course_code_authority)]
#[diesel(check_for_backend(Pg))]
pub struct CourseCodeAuthority {
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Deserialize)]
pub struct CourseCodeAuthorityCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

// #[derive(Serialize)]
// pub struct CourseCodeAuthorityReturn {
//     pub user_id: String,
//     pub user_first_name: String,
//     pub user_middle_name: String,
//     pub user_last_name: String,
//     pub user_role: String,
// }

#[derive(Deserialize)]
pub struct CourseCodeAuthorityReadOne {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct CourseCodeAuthorityReadAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct CourseCodeAuthorityUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct CourseCodeAuthorityDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name=crate::schema::fees_structure)]
#[diesel(check_for_backend(Pg))]
pub struct FeeStructure {
    pub course_name: String,
    pub year: i32,
    pub semester: i32,
    pub expenditure_name: String,
    pub expenditure_cost: i32,
}

/*
course_name VARCHAR NOT NULL,
    year INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    expenditure_name VARCHAR NOT NULL,
    expenditure_cost INTEGER NOT NULL
*/

#[derive(Serialize)]
pub struct SemExpenditureData {
    pub expenditure_name: String,
    pub expenditure_cost: i32,
}

#[derive(Serialize)]
pub struct YearExpenditureData {
    pub course_sem: i32,
    pub expenditure_name: String,
    pub expenditure_cost: i32,
}

#[derive(Serialize)]
pub struct CourseExpenditureData {
    pub course_year: i32,
    pub course_sem: i32,
    pub expenditure_name: String,
    pub expenditure_cost: i32,
}

#[derive(Deserialize)]
pub struct FeeStructureCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
    pub year: i32,
    pub semester: i32,
    pub expenditure_name: String,
    pub expenditure_cost: i32,
}

#[derive(Deserialize)]
pub struct FeeStructureGetOne {
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct FeeStructureGetOneYear {
    pub course_name: String,
    pub year: i32,
}

#[derive(Deserialize)]
pub struct FeeStructureGetOneSem {
    pub course_name: String,
    pub year: i32,
    pub sem: i32,
}

#[derive(Deserialize)]
pub struct FeeStructureGetOneExpenditure {
    pub course_name: String,
    pub year: i32,
    pub sem: i32,
    pub expenditure_name: String,
}

#[derive(Serialize)]
pub struct OneExpenditureCost {
    pub cost: i32,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name=crate::schema::fees_structure)]
#[diesel(check_for_backend(Pg))]
pub struct FeeStructureLoadAll {
    pub id: i32,
    pub course_name: String,
    pub year: i32,
    pub semester: i32,
    pub expenditure_name: String,
    pub expenditure_cost: i32,
}

#[derive(Deserialize)]
pub struct FeeStructureUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub name_course: String,
    pub course_year: i32,
    pub course_sem: i32,
    pub name_expenditure: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct FeeStructureDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub name_course: String,
    pub course_year: i32,
    pub course_sem: i32,
    pub name_expenditure: String,
}

#[derive(Deserialize)]
pub struct FeesAuthorityCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Deserialize)]
pub struct FeesAuthorityReadOne {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct FeesAuthorityUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_identity: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct FeesAuthorityDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct FeesAuthorityReadAll {
    pub owner_id: String,
    pub owner_password: String,
}

/// stores the data based on the columns on hostel_authority table.
/// stores user_id, password, first_name, middle, and last name, and role of the user
/// before being passed to the database.
#[derive(Selectable, Queryable, Insertable, Serialize)]
#[diesel(table_name=crate::schema::hostel_authority)]
#[diesel(check_for_backend(Pg))]
pub struct HostelsAuthority {
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

/// Hostels struct. Stores the data of hostels table in the database.
/// stores hostel name, number of rooms, students per room and Gender
/// It is linked with the hostels table in the db.
#[derive(Selectable, Insertable, Queryable, Serialize)]
#[diesel(table_name=crate::schema::hostels)]
#[diesel(check_for_backend(Pg))]
pub struct Hostels {
    pub hostel_name: String,
    pub no_of_rooms: i32,
    pub students_per_room: i32,
    pub gender: String,
}

/// This struct is linked with the student_hostel table in the database.
/// It stores the student's reg_no, first, middle and last name, hostel name and hostel number.
#[derive(Insertable, Selectable, Queryable, Serialize)]
#[diesel(table_name = crate::schema::student_hostel)]
#[diesel(check_for_backend(Pg))]
pub struct StudentHostel {
    pub student_reg_no: String,
    pub student_first_name: String,
    pub student_middle_name: String,
    pub student_last_name: String,
    pub hostel_name: String,
    pub hostel_room_number: i32,
    pub gender: String,
}

/// Stores data in a format similar to that of graduation table in database.
/// Links the struct to the table and checks for backend using diesel.
#[derive(Selectable, Insertable, Queryable, Serialize)]
#[diesel(table_name=crate::schema::students_graduation)]
#[diesel(check_for_backend(Pg))]
pub struct StudentsGraduation {
    pub student_reg_no: String,
    pub student_first_name: String,
    pub student_middle_name: String,
    pub student_last_name: String,
    pub student_course_year: i32,
    pub student_course: String,
    pub student_programme: String,
    pub student_department: String,
    pub student_school: String,
    pub student_class: String,
    pub student_gssp: String,
    pub student_gender: String,
    pub student_role: Option<Vec<Option<String>>>,
    pub student_hostel: Option<Vec<Option<String>>>,
    pub graduation_year: i32,
    pub graduation_month: i32,
    pub graduation_day: i32,
}

/// Takes data in the format of authority_graduation table.
/// Connects the struct to authority_graduation table; every component will match that on the authority_graduation table.
#[derive(Selectable, Insertable, Queryable, Serialize)]
#[diesel(table_name=crate::schema::authority_graduation)]
#[diesel(check_for_backend(Pg))]
pub struct AuthorityGraduation {
    pub user_id: String,
    pub user_password: String,
    pub user_role: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
}

#[derive(Deserialize)]
pub struct CreateGraduationAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_role: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
}

#[derive(Deserialize)]
pub struct GetOneGraduationAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct GetAllGraduationAuthority {
    pub owner_id: String,
    pub owner_password: String,
}
#[derive(Deserialize)]
pub struct UpdateGraduationAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct DeleteGraduationAuthority {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct GraduationStudentCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub student_reg_no: String,
    pub student_first_name: String,
    pub student_middle_name: String,
    pub student_last_name: String,
    pub student_course_year: i32,
    pub student_course: String,
    pub student_programme: String,
    pub student_department: String,
    pub student_school: String,
    pub student_class: String,
    pub student_gssp: String,
    pub student_gender: String,
    pub student_role: Option<Vec<Option<String>>>,
    pub student_hostel: Option<Vec<Option<String>>>,
    pub graduation_year: i32,
    pub graduation_month: i32,
    pub graduation_day: i32,
}

#[derive(Deserialize)]
pub struct GraduationStudentReadOne {
    pub owner_id: String,
    pub owner_password: String,
    pub student_reg_no: String,
}

#[derive(Deserialize)]
pub struct GraduationStudentReadAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct HostelsAuthorityCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Deserialize)]
pub struct HostelsAuthorityReadOne {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}
#[derive(Deserialize)]
pub struct HostelsAuthorityReadAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct HostelsAuthorityUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct StudentsHostelsCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub student_reg_no: String,
    pub student_first_name: String,
    pub student_middle_name: String,
    pub student_last_name: String,
    pub hostel_name: String,
    pub hostel_room_number: i32,
    pub gender: String,
}

#[derive(Deserialize)]
pub struct StudentsHostelsReadOne {
    pub owner_id: String,
    pub owner_password: String,
    pub student_reg_no: String,
}

#[derive(Deserialize)]
pub struct StudentsHostelsReadAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct StudentsHostelsUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct HostelsCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub hostel_name: String,
    pub no_of_rooms: i32,
    pub students_per_room: i32,
    pub gender: String,
}

#[derive(Deserialize)]
pub struct HostelsGetOne {
    pub hostel_name: String,
}

#[derive(Deserialize)]
pub struct HostelsUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub hostel_name: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct HostelsDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub hostel_name: String,
}

/// the struct connects with the course_graduation in the table storing it's field except the id
/// because the id is incremented automatically
#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name=crate::schema::course_graduation)]
#[diesel(check_for_backend(Pg))]
pub struct CourseGraduation {
    pub graduation_year: i32,
    pub course_name: String,
    pub number_students: i32,
    pub first_class_students: i32,
    pub second_class_upper_division_students: i32,
    pub second_class_lower_division_students: i32,
    pub pass: i32,
    pub fail: i32,
    pub department: String,
    pub programme: String,
}

/// Connects with the total_graduation table in the database containing all it's field.
/// It's primary key is the graduation year because there is one graduation per year.
#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name=crate::schema::total_graduation)]
#[diesel(check_for_backend(Pg))]
pub struct TotalGraduation {
    pub graduation_year: i32,
    pub number_students: i32,
    pub first_class_students: i32,
    pub second_class_upper_division_students: i32,
    pub second_class_lower_division_students: i32,
    pub pass: i32,
    pub fail: i32,
}

#[derive(Selectable, Queryable, Insertable, Serialize)]
#[diesel(table_name=crate::schema::course_graduation)]
#[diesel(check_for_backend(Pg))]
pub struct CourseGraduationReturn {
    pub id: i32,
    pub graduation_year: i32,
    pub course_name: String,
    pub number_students: i32,
    pub first_class_students: i32,
    pub second_class_upper_division_students: i32,
    pub second_class_lower_division_students: i32,
    pub pass: i32,
    pub fail: i32,
    pub department: String,
    pub programme: String,
}

/// this struct connects with the course_graduation_authority, it stores all fields that are used to be passed to the database.
/// Takes in the user_id as the table's primary key
#[derive(Selectable, Queryable, Insertable, Serialize)]
#[diesel(table_name=crate::schema::course_graduation_authority)]
#[diesel(check_for_backend(Pg))]
pub struct CourseGraduationAuthority {
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Selectable, Insertable, Queryable, Serialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name=crate::schema::course_graduation_authority)]
pub struct CourseGraduationAuthorityReturn {
    pub user_id: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationAuthorityCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub user_password: String,
    pub user_first_name: String,
    pub user_middle_name: String,
    pub user_last_name: String,
    pub user_role: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationAuthorityGetOne {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationAuthorityGetAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationAuthorityUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub user_id: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct TotalGraduationCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub graduation_year: i32,
    pub number_students: i32,
    pub first_class_students: i32,
    pub second_class_upper_division_students: i32,
    pub second_class_lower_division_students: i32,
    pub pass: i32,
    pub fail: i32,
}

#[derive(Deserialize)]
pub struct TotalGraduationGetOne {
    pub owner_id: String,
    pub owner_password: String,
    pub graduation_year: i32,
}

#[derive(Deserialize)]
pub struct TotalGraduationGetAll {
    pub owner_id: String,
    pub owner_password: String,
}

#[derive(Deserialize)]
pub struct TotalGraduationUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub graduation_year: i32,
    pub field: String,
    pub new_value: i32,
}

#[derive(Deserialize)]
pub struct CourseGraduationCreate {
    pub owner_id: String,
    pub owner_password: String,
    pub graduation_year: i32,
    pub course_name: String,
    pub number_students: i32,
    pub first_class_students: i32,
    pub second_class_upper_division_students: i32,
    pub second_class_lower_division_students: i32,
    pub pass: i32,
    pub fail: i32,
    pub department: String,
    pub programme: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationGetOne {
    pub graduation_year: i32,
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationUpdate {
    pub owner_id: String,
    pub owner_password: String,
    pub course_name: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Deserialize)]
pub struct CourseGraduationDelete {
    pub owner_id: String,
    pub owner_password: String,
    pub graduation_year: i32,
    pub course_name: String,
}
