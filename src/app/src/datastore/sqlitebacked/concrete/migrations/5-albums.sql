CREATE TABLE IF NOT EXISTS albums (
    id	INTEGER	PRIMARY KEY,
    name	TEXT	NOT NULL,
    date_number	INTEGER	NOT NULL,
    disc_total	INTEGER	NOT NULL,
    full_path	TEXT	NOT NULL,
    relative_path	TEXT	NOT NULL,
    artist_id	INTEGER	NOT NULL,

    FOREIGN KEY(artist_id)	REFERENCES artists(id)
);

CREATE UNIQUE INDEX albums_unique_name ON albums(artist_id, name);
