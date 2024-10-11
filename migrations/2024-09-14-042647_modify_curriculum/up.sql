-- Your SQL goes here
DROP TABLE IF EXISTS curriculum;
CREATE TABLE curriculum(
    unit_id VARCHAR PRIMARY KEY,
    unit_name VARCHAR NOT NULL,
    electives VARCHAR NOT NULL,
    course_name VARCHAR NOT NULL
);