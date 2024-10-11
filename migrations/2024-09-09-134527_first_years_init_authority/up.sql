-- Your SQL goes here
CREATE TABLE first_year_init_authority(
    id VARCHAR PRIMARY KEY,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);