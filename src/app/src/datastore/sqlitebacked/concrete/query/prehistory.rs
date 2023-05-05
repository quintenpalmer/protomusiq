use std::collections::BTreeMap;

use rusqlite;

pub fn get_all_historical_track_counts(main_db: &rusqlite::Connection) -> BTreeMap<u32, usize> {
    let mut statement = main_db
        .prepare("select track_id, sum(play_count) from prehistory_track_counts group by track_id")
        .unwrap();

    let results = statement
        .query_map([], map_track_id_and_count)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    results.into_iter().collect()
}

fn map_track_id_and_count(row: &rusqlite::Row) -> rusqlite::Result<(u32, usize)> {
    Ok((row.get(0)?, {
        let x: u32 = row.get(1)?;
        x as usize
    }))
}

pub fn get_historical_track_count(main_db: &rusqlite::Connection, track_id: u32) -> usize {
    let mut statement = main_db
        .prepare("SELECT * FROM prehistory_track_counts WHERE track_id = ?")
        .unwrap();

    let results = statement
        .query_map(rusqlite::params![track_id], map_prehistory_record)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    let mut total_play_count = 0;

    for (_track_id, _source_id, count) in results.into_iter() {
        total_play_count += count;
    }

    total_play_count
}

fn map_prehistory_record(row: &rusqlite::Row) -> rusqlite::Result<(u32, u32, usize)> {
    Ok((row.get(0)?, row.get(1)?, {
        let x: u32 = row.get(2)?;
        x as usize
    }))
}
