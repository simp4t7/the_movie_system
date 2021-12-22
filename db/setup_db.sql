CREATE TABLE users
(
    id TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT not null,
    salt TEXT not null,
    groups TEXT,
    date_created DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    date_modified DATETIME DEFAULT (DATETIME('now')) NOT NULL
);

CREATE TABLE groups
(
    id TEXT NOT NULL UNIQUE,
    members TEXT NOT NULL,
    movies_watched TEXT,
    current_movies TEXT,
    turn TEXT,
    date_created DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    date_modified DATETIME DEFAULT (DATETIME('now')) NOT NULL
);
