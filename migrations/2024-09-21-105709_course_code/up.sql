-- Your SQL goes here
DROP TABLE IF EXISTS course_code;
CREATE TABLE course_codes(
    course_name VARCHAR PRIMARY KEY,
    course_code VARCHAR NOT NULL
);