-- Your SQL goes here
DROP TABLE IF EXISTS students_graduation;
CREATE TABLE students_graduation(
    student_reg_no VARCHAR PRIMARY KEY,
    student_first_name VARCHAR NOT NULL,
    student_middle_name VARCHAR NOT NULL,
    student_last_name VARCHAR NOT NULL,
    student_course_year INTEGER NOT NULL,
    student_course VARCHAR NOT NULL,
    student_programme VARCHAR NOT NULL,
    student_department VARCHAR  NOT NULL,
    student_school VARCHAR NOT NULL,
    student_class VARCHAR NOT NULL,
    student_gssp VARCHAR NOT NULL,
    student_gender VARCHAR NOT NULL,
    student_role TEXT[],
    student_hostel TEXT[],
    graduation_year INTEGER NOT NULL,
    graduation_month INTEGER NOT NULL,
    graduation_day INTEGER NOT NULL
);