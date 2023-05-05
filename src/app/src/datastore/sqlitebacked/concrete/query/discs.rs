use rusqlite;

use super::super::dbmodel;

pub fn insert_disc(main_db: &rusqlite::Connection, album_id: u32, disc_no: u32) -> u32 {
    main_db
        .query_row(
            "INSERT INTO discs VALUES (
            ?,
            ?,
            ?,
            ?) RETURNING id",
            rusqlite::params![
                rusqlite::types::Null,
                disc_no,
                rusqlite::types::Null,
                album_id
            ],
            |row| row.get(0),
        )
        .unwrap()
}

pub fn select_discs(main_db: &rusqlite::Connection) -> Vec<dbmodel::Disc> {
    let mut statement = main_db.prepare("SELECT * FROM discs").unwrap();
    let discs = statement
        .query_map([], map_disc)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    discs
}

fn map_disc(row: &rusqlite::Row) -> rusqlite::Result<dbmodel::Disc> {
    Ok(dbmodel::Disc {
        id: row.get(0)?,
        disc_no: row.get(1)?,
        name: row.get(2)?,
        album_id: row.get(3)?,
    })
}
