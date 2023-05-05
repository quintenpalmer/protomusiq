CREATE TABLE IF NOT EXISTS album_genres (
    album_id	INTEGER	NOT NULL,
    genre_id	INTEGER	NOT NULL,
    priority	INTEGER	NOT NULL,

    FOREIGN KEY(album_id)	REFERENCES albums(id),
    FOREIGN KEY(genre_id)	REFERENCES genres(id)
);

CREATE UNIQUE INDEX album_genres_unique_tuple ON album_genres(album_id, genre_id);
