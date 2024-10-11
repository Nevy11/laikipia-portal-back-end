-- Your SQL goes here
CREATE TABLE students_information(
    reg_no VARCHAR PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    year_of_study INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    programme VARCHAR NOT NULL,
    COURSE VARCHAR NOT NULL,
    admission_date INTEGER NOT NULL,
    admission_month INTEGER NOT NULL,
    admission_year INTEGER NOT NULL
);