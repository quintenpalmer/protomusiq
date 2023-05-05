CREATE TABLE IF NOT EXISTS artists (
    id	INTEGER	PRIMARY KEY,
    name	TEXT	NOT NULL
);

CREATE UNIQUE INDEX artists_unique_name ON artists(name);
