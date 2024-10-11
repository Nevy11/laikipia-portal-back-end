-- Your SQL goes here
CREATE TABLE staff(
    user_id VARCHAR PRIMARY KEY,
    user_password VARCHAR NOT NULL,
    user_first_name VARCHAR NOT NULL,
    user_middle_name VARCHAR NOT NULL,
    user_last_name VARCHAR NOT NULL,
    user_age INTEGER NOT NULL,
    department VARCHAR NOT NULL,
    user_role VARCHAR NOT NULL,
    user_position VARCHAR NOT NULL
);