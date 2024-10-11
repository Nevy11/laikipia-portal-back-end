-- Your SQL goes here
CREATE TABLE student_hostel(
    student_reg_no VARCHAR PRIMARY KEY,
    student_first_name VARCHAR NOT NULL,
    student_middle_name VARCHAR NOT NULL,
    student_last_name VARCHAR NOT NULL,
    hostel_name VARCHAR NOT NULL,
    hostel_room_number INTEGER NOT NULL,
    gender VARCHAR NOT NULL
);

CREATE TABLE hostels(
    hostel_name VARCHAR PRIMARY KEY,
    no_of_rooms INTEGER NOT NULL,
    students_per_room INTEGER NOT NULL,
    gender VARCHAR NOT NULL
);