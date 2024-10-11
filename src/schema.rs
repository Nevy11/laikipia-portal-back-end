// @generated automatically by Diesel CLI.

diesel::table! {
    access_to_course_duration (id) {
        id -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    authority_graduation (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_role -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
    }
}

diesel::table! {
    course_code_authority (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
        user_role -> Varchar,
    }
}

diesel::table! {
    course_codes (course_name) {
        course_name -> Varchar,
        course_code -> Varchar,
    }
}

diesel::table! {
    course_duration (course_name) {
        course_name -> Varchar,
        course_length -> Int4,
    }
}

diesel::table! {
    course_graduation (id) {
        id -> Int4,
        graduation_year -> Int4,
        course_name -> Varchar,
        number_students -> Int4,
        first_class_students -> Int4,
        second_class_upper_division_students -> Int4,
        second_class_lower_division_students -> Int4,
        pass -> Int4,
        fail -> Int4,
        department -> Varchar,
        programme -> Varchar,
    }
}

diesel::table! {
    course_graduation_authority (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
        user_role -> Varchar,
    }
}

diesel::table! {
    curriculum (unit_id) {
        unit_id -> Varchar,
        unit_name -> Varchar,
        electives -> Varchar,
        course_name -> Varchar,
    }
}

diesel::table! {
    curriculum_authority (owner_id) {
        owner_id -> Varchar,
        owner_password -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        owner_role -> Varchar,
    }
}

diesel::table! {
    fees_authority (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
        user_role -> Varchar,
    }
}

diesel::table! {
    fees_structure (id) {
        id -> Int4,
        course_name -> Varchar,
        year -> Int4,
        semester -> Int4,
        expenditure_name -> Varchar,
        expenditure_cost -> Int4,
    }
}

diesel::table! {
    first_year_init_authority (id) {
        id -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    graduation_authority (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
        user_role -> Varchar,
    }
}

diesel::table! {
    hostel_authority (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
        user_role -> Varchar,
    }
}

diesel::table! {
    hostels (hostel_name) {
        hostel_name -> Varchar,
        no_of_rooms -> Int4,
        students_per_room -> Int4,
        gender -> Varchar,
    }
}

diesel::table! {
    login (reg_no) {
        reg_no -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    staff (user_id) {
        user_id -> Varchar,
        user_password -> Varchar,
        user_first_name -> Varchar,
        user_middle_name -> Varchar,
        user_last_name -> Varchar,
        user_age -> Int4,
        department -> Varchar,
        user_role -> Varchar,
        user_position -> Varchar,
    }
}

diesel::table! {
    student_hostel (student_reg_no) {
        student_reg_no -> Varchar,
        student_first_name -> Varchar,
        student_middle_name -> Varchar,
        student_last_name -> Varchar,
        hostel_name -> Varchar,
        hostel_room_number -> Int4,
        gender -> Varchar,
    }
}

diesel::table! {
    students (reg_no) {
        reg_no -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        year_of_study -> Int4,
        semester -> Int4,
        course -> Varchar,
        programme -> Varchar,
        department -> Varchar,
        school -> Varchar,
        class -> Varchar,
        gssp -> Varchar,
        gender -> Varchar,
        students_role -> Varchar,
        admission_date -> Int4,
        admission_month -> Int4,
        admission_year -> Int4,
    }
}

diesel::table! {
    students_graduation (student_reg_no) {
        student_reg_no -> Varchar,
        student_first_name -> Varchar,
        student_middle_name -> Varchar,
        student_last_name -> Varchar,
        student_course_year -> Int4,
        student_course -> Varchar,
        student_programme -> Varchar,
        student_department -> Varchar,
        student_school -> Varchar,
        student_class -> Varchar,
        student_gssp -> Varchar,
        student_gender -> Varchar,
        student_role -> Nullable<Array<Nullable<Text>>>,
        student_hostel -> Nullable<Array<Nullable<Text>>>,
        graduation_year -> Int4,
        graduation_month -> Int4,
        graduation_day -> Int4,
    }
}

diesel::table! {
    total_graduation (graduation_year) {
        graduation_year -> Int4,
        number_students -> Int4,
        first_class_students -> Int4,
        second_class_upper_division_students -> Int4,
        second_class_lower_division_students -> Int4,
        pass -> Int4,
        fail -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    access_to_course_duration,
    authority_graduation,
    course_code_authority,
    course_codes,
    course_duration,
    course_graduation,
    course_graduation_authority,
    curriculum,
    curriculum_authority,
    fees_authority,
    fees_structure,
    first_year_init_authority,
    graduation_authority,
    hostel_authority,
    hostels,
    login,
    staff,
    student_hostel,
    students,
    students_graduation,
    total_graduation,
);
