CREATE TABLE IF NOT EXISTS discs (
    id	INTEGER	PRIMARY KEY,
    disc_no	INTEGER	NOT NULL,
    name	TEXT,
    album_id	INTEGER	NOT NULL,

    FOREIGN KEY(album_id)	REFERENCES albums(id)
);

CREATE UNIQUE INDEX disc_unique_per_album ON discs(album_id, disc_no);
