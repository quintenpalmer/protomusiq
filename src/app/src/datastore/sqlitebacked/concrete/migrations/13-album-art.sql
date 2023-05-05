CREATE TABLE IF NOT EXISTS album_art (
    album_id	INTEGER	NOT NULL,
    image_size_enum	TEXT	NOT NULL,
    image_bytes	BLOB	NOT NULL,

    FOREIGN KEY(album_id)	REFERENCES albums(id)
);

CREATE UNIQUE INDEX album_art_unique_per_size ON album_art(album_id, image_size_enum);
