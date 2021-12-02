CREATE TABLE users
(
    id TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT not null,
    salt TEXT not null,
    date_created DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    date_modified DATETIME DEFAULT (DATETIME('now')) NOT NULL
);
