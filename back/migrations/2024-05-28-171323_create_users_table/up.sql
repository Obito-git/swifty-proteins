CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL CHECK (LENGTH(username) >= 2 AND LENGTH(username) <= 30),
    password VARCHAR(255) NOT NULL
);
