CREATE TABLE IF NOT EXISTS prehistory_track_counts (
    track_id	INTEGER	NOT NULL,
    source_id	INTEGER	NOT NULL,
    play_count	INTEGER	NOT NULL,

    FOREIGN KEY(track_id)	REFERENCES tracks(id),
    FOREIGN KEY(source_id)	REFERENCES sources(id)
);
