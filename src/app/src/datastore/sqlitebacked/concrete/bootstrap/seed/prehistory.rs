use crate::model;
use crate::util::logging;

use super::super::super::constants;
use super::super::super::dbmodel::bridge;

pub fn seed_prehistory(
    main_db: &mut rusqlite::Connection,
    library: &bridge::SixtyFourLibrary,
    historical_data: &Vec<model::PrehistoryRecord>,
) {
    let mut logger = logging::Logger::new(logging::LogType::Timing, "seeding prehistory");

    let tx = main_db.transaction().unwrap();

    for prehistory_record in historical_data.iter() {
        println!("about to insert {:?}", prehistory_record);
        insert_prehistory_record(&tx, &library, &prehistory_record);
        logger.print_elapsed(format!("inserting track {:?}", prehistory_record));
    }

    tx.execute(
        "INSERT INTO data_migrations VALUES (
                ?,
                ?)",
        rusqlite::params![
            rusqlite::types::Null,
            constants::Migration::Prehistory.get_name()
        ],
    )
    .unwrap();

    tx.commit().unwrap();
}

fn insert_prehistory_record(
    main_db: &rusqlite::Connection,
    library: &bridge::SixtyFourLibrary,
    record: &model::PrehistoryRecord,
) {
    let track = library.track_from_unique_key(&record.key);
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        record.source,
        record.key.track_no,
        record.key.disc_no,
        record.key.album_id,
        record.key.artist_id,
        record.count,
    );
    main_db
        .execute(
            "INSERT INTO prehistory_track_counts VALUES (
            ?,
            (SELECT id FROM sources WHERE name = ?),
            ?)",
            rusqlite::params![track.id, record.source, record.count],
        )
        .unwrap();
}
