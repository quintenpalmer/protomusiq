CREATE TABLE IF NOT EXISTS live_track_history (
    track_id	INTEGER	NOT NULL,
    listened_date	TEXT	NOT NULL,

    FOREIGN KEY(track_id)	REFERENCES tracks(id)
);
