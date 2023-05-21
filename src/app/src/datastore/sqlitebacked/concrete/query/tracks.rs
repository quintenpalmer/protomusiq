use std::path;
use std::time;

use rusqlite;

use musiqlibrary;

use super::super::dbmodel;

pub fn insert_track(
    main_db: &rusqlite::Connection,
    disc_id: u32,
    track: &musiqlibrary::FullTrackMetadata,
) -> u32 {
    main_db
        .query_row(
            "INSERT INTO tracks VALUES (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?) RETURNING id",
            rusqlite::params![
                rusqlite::types::Null,
                track.track,
                track.title,
                disc_id,
                rusqlite::types::Null,
                track.duration.as_secs(),
                track.path.clone().into_os_string().into_string().unwrap(),
                track
                    .relative_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                track
                    .last_modified
                    .duration_since(time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                track.ext.clone(),
                rusqlite::types::Null,
            ],
            |row| row.get(0),
        )
        .unwrap()
}

pub fn select_tracks(main_db: &rusqlite::Connection) -> Vec<dbmodel::Track> {
    let mut statement = main_db
        .prepare("SELECT * FROM tracks WHERE deleted_at IS NULL")
        .unwrap();
    let tracks = statement
        .query_map([], map_track)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    tracks
}

fn map_track(row: &rusqlite::Row) -> rusqlite::Result<dbmodel::Track> {
    Ok(dbmodel::Track {
        id: row.get(0)?,
        track_no: row.get(1)?,
        track_name: row.get(2)?,
        disc_id: row.get(3)?,

        genre_id: row.get(4)?,
        duration: time::Duration::from_secs(row.get(5)?),
        full_path: path::PathBuf::from({
            let s: String = row.get(6)?;
            s
        }),
        relative_path: path::PathBuf::from({
            let s: String = row.get(7)?;
            s
        }),
        last_modified: time::SystemTime::UNIX_EPOCH + time::Duration::from_secs(row.get(8)?),
        ext: row.get(9)?,
    })
}
