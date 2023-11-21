-- Your SQL goes here
CREATE TABLE courses (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)