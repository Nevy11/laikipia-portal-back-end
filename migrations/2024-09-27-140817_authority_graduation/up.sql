-- Your SQL goes here
CREATE TABLE authority_graduation(
  user_id VARCHAR PRIMARY KEY,
  user_password VARCHAR NOT NULL,
  user_role VARCHAR NOT NULL,
  user_first_name VARCHAR NOT NULL,
  user_middle_name VARCHAR NOT NULL,
  user_last_name VARCHAR NOT NULL  
);