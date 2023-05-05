CREATE TABLE IF NOT EXISTS raw_tracks (
    id	INTEGER	PRIMARY KEY,
    title	TEXT	NOT NULL,
    track	INTEGER	NOT NULL,
    raw_track	INTEGER,
    disc	INTEGER	NOT NULL,
    raw_disc	INTEGER,
    disc_total	INTEGER,
    album	TEXT	NOT NULL,
    raw_album	TEXT,
    album_artist	TEXT	NOT NULL,
    track_artist	TEXT	NOT NULL,
    genre	TEXT	NOT NULL,
    date_number	INTEGER	NOT NULL,
    raw_date	TEXT	NOT NULL,
    duration	INTEGER	NOT NULL,
    path	TEXT	NOT NULL,
    relative_path	TEXT	NOT NULL,
    last_modified	INTEGER	NOT NULL,
    ext	TEXT	NOT NULL
);

CREATE UNIQUE INDEX raw_tracks_unique_tuple ON raw_tracks(track, disc, album, album_artist);
