-- Your SQL goes here
CREATE TABLE bicycles (
    id SERIAL PRIMARY KEY,
    wheel_size INT NOT NULL,
    description TEXT NOT NULL
);