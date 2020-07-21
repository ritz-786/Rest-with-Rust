-- Your SQL goes here
CREATE TABLE "teachers"
(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    designation VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    salary INT NOT NULL,
    age INT NOT NULL
)