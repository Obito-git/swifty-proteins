CREATE TABLE IF NOT EXISTS users
(
    id       INTEGER NOT NULL PRIMARY KEY,
    username TEXT    NOT NULL UNIQUE CHECK (LENGTH(username) >= 2 AND LENGTH(username) <= 30),
    password TEXT    NOT NULL
);
