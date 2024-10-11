-- Your SQL goes here
CREATE TABLE course_graduation(
    id SERIAL PRIMARY KEY,
    graduation_year INTEGER NOT NULL,
    course_name VARCHAR NOT NULL,
    number_students INTEGER NOT NULL,
    first_class_students INTEGER NOT NULL,
    second_class_upper_division_students INTEGER NOT NULL,
    second_class_lower_division_students INTEGER NOT NULL,
    pass INTEGER NOT NULL,
    fail INTEGER NOT NULL,
    department VARCHAR NOT NULL,
    programme VARCHAR NOT NULL
);

CREATE TABLE total_graduation(
    graduation_year INTEGER PRIMARY KEY,
    number_students INTEGER NOT NULL,
    first_class_students INTEGER NOT NULL,
    second_class_upper_division_students INTEGER NOT NULL,
    second_class_lower_division_students INTEGER NOT NULL,
    pass INTEGER NOT NULL,
    fail INTEGER NOT NULL
);
