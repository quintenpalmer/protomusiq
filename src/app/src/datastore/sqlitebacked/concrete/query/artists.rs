use rusqlite;

use musiqlibrary;

use super::super::dbmodel;

pub fn insert_artist(main_db: &rusqlite::Connection, artist: &musiqlibrary::ArtistInfo) -> u32 {
    main_db
        .query_row(
            "INSERT INTO artists VALUES (
            ?,
            ?) RETURNING id",
            rusqlite::params![rusqlite::types::Null, artist.artist_name],
            |row| row.get(0),
        )
        .unwrap()
}

pub fn select_artists(main_db: &rusqlite::Connection) -> Vec<dbmodel::Artist> {
    let mut statement = main_db.prepare("SELECT * FROM artists").unwrap();
    let artists = statement
        .query_map([], map_artist)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    artists
}

fn map_artist(row: &rusqlite::Row) -> rusqlite::Result<dbmodel::Artist> {
    Ok(dbmodel::Artist {
        id: row.get(0)?,
        name: row.get(1)?,
    })
}
