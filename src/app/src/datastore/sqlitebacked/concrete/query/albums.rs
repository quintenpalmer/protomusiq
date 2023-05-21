use std::path;

use rusqlite;

use musiqlibrary;

use super::super::dbmodel;

pub fn insert_album(
    main_db: &rusqlite::Connection,
    artist_id: u32,
    disc_total: u32,
    album: &musiqlibrary::AlbumInfo,
) -> u32 {
    main_db
        .query_row(
            "INSERT INTO albums VALUES (
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?) RETURNING id",
            rusqlite::params![
                rusqlite::types::Null,
                album.album_name,
                album.start_date,
                disc_total,
                album.path.clone().into_os_string().into_string().unwrap(),
                album
                    .relative_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                artist_id
            ],
            |row| row.get(0),
        )
        .unwrap()
}

pub fn select_albums(main_db: &rusqlite::Connection) -> Vec<dbmodel::Album> {
    let mut statement = main_db
        .prepare("SELECT * FROM albums WHERE deleted_at IS NULL")
        .unwrap();
    let albums = statement
        .query_map([], map_album)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    albums
}

fn map_album(row: &rusqlite::Row) -> rusqlite::Result<dbmodel::Album> {
    Ok(dbmodel::Album {
        id: row.get(0)?,
        name: row.get(1)?,
        date_number: row.get(2)?,
        disc_total: row.get(3)?,
        full_path: path::PathBuf::from({
            let s: String = row.get(4)?;
            s
        }),
        relative_path: path::PathBuf::from({
            let s: String = row.get(5)?;
            s
        }),
        artist_id: row.get(6)?,
    })
}
