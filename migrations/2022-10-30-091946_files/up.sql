-- Your SQL goes here

CREATE TABLE files
(
    id        BiGSERIAL PRIMARY KEY,
    owner     BIGINT    NOT NULL,
    name      TEXT      NOT NULL,
    size      INTEGER   NOT NULL,
    parent      TEXT      NOT NULL DEFAULT '/',
    file_type INTEGER   NOT NULL,
    create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    meta      TEXT      NOT NULL DEFAULT '',
    UNIQUE (owner, name, parent)
)