use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use crate::util::logging;

use super::super::super::constants;
use super::super::super::dbmodel::{self, bridge};

pub fn seed_livehistory(
    main_db: &mut rusqlite::Connection,
    library: &bridge::SixtyFourLibrary,
    livehistory_map: &BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>>,
) {
    let mut logger = logging::Logger::new(logging::LogType::Timing, "seeding livehistory");

    let tx = main_db.transaction().unwrap();

    for (track_id, date_times) in livehistory_map.iter() {
        for date_time in date_times.iter() {
            println!("{:?}\n{:?}", track_id, date_time);
            let livehistory_record = dbmodel::LivehistoryRecord {
                track_id: library.track_from_unique_key(&track_id).id,
                listened_date: date_time.clone(),
            };
            println!("{:?}", livehistory_record.track_id);
            insert_livehistory_record(&tx, &livehistory_record);
            logger.print_elapsed(format!("inserting track {:?}", livehistory_record));
        }
    }

    tx.execute(
        "INSERT INTO data_migrations VALUES (
                ?,
                ?)",
        rusqlite::params![
            rusqlite::types::Null,
            constants::Migration::Livehistory.get_name()
        ],
    )
    .unwrap();

    tx.commit().unwrap()
}

fn insert_livehistory_record(main_db: &rusqlite::Connection, record: &dbmodel::LivehistoryRecord) {
    main_db
        .execute(
            "INSERT INTO live_track_history VALUES (
            ?,
            ?)",
            rusqlite::params![record.track_id, record.listened_date],
        )
        .unwrap();
}
