-- Your SQL goes here

CREATE TABLE assets
(
    id        BIGSERIAL PRIMARY KEY,
    size      INTEGER NOT NULL,
    hash      TEXT UNIQUE NOT NULL,
    owner_ref INTEGER NOT NULL,
    create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);