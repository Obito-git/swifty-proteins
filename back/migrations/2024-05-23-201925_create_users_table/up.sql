CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL CHECK (LENGTH(name) >= 2 AND LENGTH(name) <= 30),
    password VARCHAR(255) NOT NULL,
    login VARCHAR(30) NOT NULL UNIQUE CHECK (LENGTH(login) >= 2 AND LENGTH(login) <= 30)
);
