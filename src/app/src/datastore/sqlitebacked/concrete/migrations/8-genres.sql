CREATE TABLE IF NOT EXISTS genres (
    id	INTEGER	PRIMARY KEY,
    name	TEXT	NOT NULL
);

CREATE UNIQUE INDEX genres_unique_name ON genres(name);
