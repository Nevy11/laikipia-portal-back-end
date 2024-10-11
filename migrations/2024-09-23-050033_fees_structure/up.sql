-- Your SQL goes here
CREATE TABLE fees_structure(                                                                                                                                                               
    id SERIAL PRIMARY KEY,                                            
    course_name VARCHAR NOT NULL,
    year INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    expenditure_name VARCHAR NOT NULL,
    expenditure_cost INTEGER NOT NULL
);