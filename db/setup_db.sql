CREATE TABLE users
(
    id TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT not null,
    salt TEXT not null,
    date_created DATETIME with time zone not null,
    date_modified TIMESTAMP with time zone not null
);
