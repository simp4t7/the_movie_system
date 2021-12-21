CREATE TABLE users
(
    id TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT not null,
    salt TEXT not null,
    groups BLOB,
    date_created DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    date_modified DATETIME DEFAULT (DATETIME('now')) NOT NULL
);

CREATE TABLE groups
(
    id TEXT NOT NULL UNIQUE,
    members BLOB NOT NULL,
    movies_watched BLOB,
    date_created DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    date_modified DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    current_movies BLOB,
    turn TEXT
);
