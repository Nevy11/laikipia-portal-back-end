-- Your SQL goes here
CREATE TABLE curriculum_authority(
    owner_id VARCHAR PRIMARY KEY,
    owner_password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    owner_role VARCHAR NOT NULL
);