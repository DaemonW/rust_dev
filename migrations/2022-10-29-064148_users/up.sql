-- Your SQL goes here

CREATE TABLE users
(
    id       BIGSERIAL PRIMARY KEY,
    username TEXT    NOT NULL UNIQUE,
    password TEXT    NOT NULL,
    phone    TEXT    NOT NULL,
    email    TEXT    NOT NULL,
    role     INTEGER NOT NULL,
    status   INTEGER NOT NULL,
    info     JSONB   NOT NULL
)