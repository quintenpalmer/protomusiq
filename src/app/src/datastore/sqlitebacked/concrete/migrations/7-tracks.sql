CREATE TABLE IF NOT EXISTS tracks (
    id	INTEGER	PRIMARY KEY,
    track_no	INTEGER	NOT NULL,
    track_name	TEXT	NOT NULL,
    disc_id	INTEGER	NOT NULL,

    genre_id	INTEGER,
    duration	INTEGER	NOT NULL,
    full_path	TEXT	NOT NULL,
    relative_path	TEXT	NOT NULL,
    last_modified	INTEGER	NOT NULL,
    ext	TEXT	NOT NULL,

    FOREIGN KEY(disc_id)	REFERENCES discs(id),
    FOREIGN KEY(genre_id)	REFERENCES genres(id)
);

CREATE UNIQUE INDEX track_unique_number ON tracks(disc_id, track_no);
