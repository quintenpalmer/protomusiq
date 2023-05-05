use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use rusqlite;

use super::super::dbmodel::bridge;

pub fn get_all_live_track_counts(main_db: &rusqlite::Connection) -> BTreeMap<u32, usize> {
    let mut statement = main_db
        .prepare("select track_id, count(listened_date) from live_track_history group by track_id")
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

pub fn get_live_track_count(main_db: &rusqlite::Connection, track_id: u32) -> usize {
    let mut statement = main_db
        .prepare("SELECT count(*) FROM live_track_history WHERE track_id = ?")
        .unwrap();

    let results = statement
        .query_map(rusqlite::params![track_id], map_track_count)
        .unwrap()
        .into_iter()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    let mut total_play_count = 0;

    for count in results.into_iter() {
        total_play_count += count;
    }

    total_play_count
}

fn map_track_count(row: &rusqlite::Row) -> rusqlite::Result<usize> {
    Ok({
        let x: u32 = row.get(0)?;
        x as usize
    })
}

pub fn increment_tracks_with_dates(
    main_db: &mut rusqlite::Connection,
    bridge: &bridge::SixtyFourLibrary,
    tracks_with_dates: Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)>,
) {
    let tx = main_db.transaction().unwrap();

    for (track, datetime) in tracks_with_dates.into_iter() {
        insert_track_with_date(&tx, &bridge, track, datetime);
    }

    tx.commit().unwrap();
}

pub fn insert_track_with_date(
    main_db: &rusqlite::Connection,
    bridge: &bridge::SixtyFourLibrary,
    track: musiqlibrary::FullTrackMetadata,
    datetime: DateTime<Local>,
) {
    let db_track =
        bridge.track_from_unique_key(&musiqlibrary::TrackUniqueIdentifier::from_track(&track));

    main_db
        .execute(
            "INSERT INTO live_track_history VALUES (
            ?,
            ?)",
            rusqlite::params![db_track.id, datetime],
        )
        .unwrap();
}
