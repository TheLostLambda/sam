-- Your SQL goes here
CREATE TABLE students (
 name text NOT NULL UNIQUE,
 number integer PRIMARY KEY,
 year integer NOT NULL,
 peak text NOT NULL,
 secret text DEFAULT NULL
);
