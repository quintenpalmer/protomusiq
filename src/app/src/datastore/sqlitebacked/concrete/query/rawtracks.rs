use std::path;
use std::time;

use super::super::util::{option_to_null, sql_row_to_id};

#[allow(dead_code)]
pub fn get_tracks(conn: &rusqlite::Connection) -> Vec<musiqlibrary::FullTrackMetadata> {
    let mut statement = conn.prepare("SELECT * FROM raw_tracks").unwrap();
    let tracks = statement
        .query_map([], map_track_from_result)
        .unwrap()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();
    tracks
}

fn map_track_from_result(
    line: &rusqlite::Row,
) -> rusqlite::Result<musiqlibrary::FullTrackMetadata> {
    Ok(musiqlibrary::FullTrackMetadata {
        title: line.get(1)?,
        track: line.get(2)?,
        raw_track: line.get(3)?,
        disc: line.get(4)?,
        raw_disc: line.get(5)?,
        disc_total: line.get(6)?,
        album: line.get(7)?,
        raw_album: line.get(8)?,
        album_id: sql_row_to_id(&line.get(7)?),
        album_artist: line.get(9)?,
        album_artist_id: sql_row_to_id(&line.get(9)?),
        track_artist: line.get(10)?,
        track_artist_id: sql_row_to_id(&line.get(10)?),
        genre: line.get(11)?,
        date_number: line.get(12)?,
        raw_date: line.get(13)?,
        duration: time::Duration::from_secs(line.get(14)?),
        path: path::PathBuf::from({
            let s: String = line.get(15)?;
            s
        }),
        relative_path: path::PathBuf::from({
            let s: String = line.get(16)?;
            s
        }),
        last_modified: time::SystemTime::UNIX_EPOCH + time::Duration::from_secs(line.get(17)?),
        ext: line.get(18)?,
    })
}

pub fn insert_raw_track(main_db: &rusqlite::Connection, track: &musiqlibrary::FullTrackMetadata) {
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
        track.title.clone(),
        track.track,
        track.disc,
        track.album.clone(),
        track.album_artist.clone(),
        track.path.clone().into_os_string().into_string().unwrap(),
    );
    main_db
        .execute(
            "INSERT INTO raw_tracks VALUES (
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
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?)",
            rusqlite::params![
                rusqlite::types::Null,
                track.title.clone(),
                track.track,
                option_to_null(&track.raw_track),
                track.disc,
                option_to_null(&track.raw_disc),
                option_to_null(&track.disc_total),
                track.album.clone(),
                option_to_null(&track.raw_album),
                track.album_artist.clone(),
                track.track_artist.clone(),
                track.genre.clone(),
                track.date_number,
                track.raw_date.clone(),
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
            ],
        )
        .unwrap();
}
