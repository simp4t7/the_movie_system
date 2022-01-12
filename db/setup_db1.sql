-- set up in a new movie site db
-- users no change
-- groups: includes info of group + system
--	movies_watched: list of movie_watched { movie_name, movie_link, date_watched }
--	movies_picked: list of movie_picked { movie_name, movie_link, picked_by, already_watched_by? }
--	system_state: either adding_moves | picking_moves | movie_chosen

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
    name TEXT NOT NULL,
    members TEXT NOT NULL,
    movies_watched TEXT,
    date_created DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    date_modified DATETIME DEFAULT (DATETIME('now')) NOT NULL,
    system_state TEXT NOT NULL,
    picking_turn TEXT,
    movies_picked TEXT
);

