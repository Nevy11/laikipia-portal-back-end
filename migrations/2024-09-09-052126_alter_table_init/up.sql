-- Your SQL goes here
DROP TABLE IF EXISTS student_init;
CREATE TABLE student_init (
    reg_no VARCHAR PRIMARY KEY ,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    year_of_study INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    programme VARCHAR NOT NULL,
    course VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    school VARCHAR NOT NULL,
    class VARCHAR NOT NULL,
    GSSP VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    admission_date INTEGER NOT NULL,
    admission_month INTEGER NOT NULL,
    admission_year INTEGER NOT NULL
);


DROP TABLE IF EXISTS students_information;
CREATE TABLE students_information(
    reg_no VARCHAR PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    year_of_study INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    programme VARCHAR NOT NULL,
    course VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    school VARCHAR NOT NULL,
    class VARCHAR NOT NULL,
    gssp VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    admission_date INTEGER NOT NULL,
    admission_month INTEGER NOT NULL,
    admission_year INTEGER NOT NULL
);
