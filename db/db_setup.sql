CREATE TABLE users
(
    username TEXT NOT NULL UNIQUE,
    data TEXT NOT NULL
);

CREATE TABLE groups
(
    id TEXT NOT NULL UNIQUE,
    data TEXT NOT NULL
);
