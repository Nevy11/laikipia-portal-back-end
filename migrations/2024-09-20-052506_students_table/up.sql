-- Your SQL goes here
CREATE TABLE students(
    reg_no VARCHAR PRIMARY KEY,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    year_of_study INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    course VARCHAR NOT NULL,
    programme VARCHAR NOT NULL,
    department VARCHAR  NOT NULL,
    school VARCHAR NOT NULL,
    class VARCHAR NOT NULL,
    gssp VARCHAR NOT NULL,
    gender VARCHAR NOT NULL,
    students_role VARCHAR NOT NULL,
    admission_date INTEGER NOT NULL,
    admission_month INTEGER NOT NULL,
    admission_year INTEGER NOT NULL
);