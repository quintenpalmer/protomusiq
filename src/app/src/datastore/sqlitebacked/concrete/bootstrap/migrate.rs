use std::cmp;
use std::fs;

use crate::model;

use crate::datastore::localfs;

pub fn create_all_tables(config_state: model::app::AppConfigState, main_db: &rusqlite::Connection) {
    let app_data_path = config_state.app_data_path.clone();

    let migrations_to_process_path =
        localfs::build_tree_for_dirs(&app_data_path, vec!["dbs", "migrations", "toprocess"]);

    let mut files: Vec<_> = fs::read_dir(migrations_to_process_path)
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect();
    files.sort_by(leading_number_sort);

    for file in files.into_iter() {
        let migration_number = get_leading_number_of_migration_filename(&file);

        let has_migration = check_has_migration_number(&main_db, migration_number);

        if !has_migration {
            let migration_sql = fs::read_to_string(file.path()).unwrap();

            main_db.execute_batch(&migration_sql).unwrap();
            mark_migration_number(&main_db, migration_number);

            println!(
                "just applied migration {} ({:?})",
                migration_number,
                file.path().file_name()
            );
        } else {
            println!(
                "already had migration {} ({:?})",
                migration_number,
                file.path().file_name()
            );
        }
    }

    insert_all_sources(&main_db);
}

fn check_has_migration_number(main_db: &rusqlite::Connection, number: u32) -> bool {
    let count: u32 = main_db
        .query_row(
            "SELECT count(*) FROM schema_migrations WHERE migration_number = ?",
            rusqlite::params![number],
            |row| row.get(0),
        )
        .unwrap();

    count != 0
}

fn mark_migration_number(main_db: &rusqlite::Connection, number: u32) {
    main_db
        .execute(
            "INSERT INTO schema_migrations VALUES (?, ?)",
            rusqlite::params![rusqlite::types::Null, number],
        )
        .unwrap();
}

fn insert_all_sources(main_db: &rusqlite::Connection) {
    let num_sources: u32 = main_db
        .query_row("SELECT count(*) FROM sources", [], |row| row.get(0))
        .unwrap();

    if num_sources == 0 {
        for name in &["Google Play Music All Access", "Spotify", "Jellyfin", "YouTube"] {
            main_db
                .execute(
                    "INSERT INTO sources VALUES (
                ?,
                ?)",
                    rusqlite::params![rusqlite::types::Null, name,],
                )
                .unwrap();
        }
    }
}

fn leading_number_sort(first: &fs::DirEntry, second: &fs::DirEntry) -> cmp::Ordering {
    let first_number = get_leading_number_of_migration_filename(&first);
    let second_number = get_leading_number_of_migration_filename(&second);

    first_number.cmp(&second_number)
}

fn get_leading_number_of_migration_filename(entry: &fs::DirEntry) -> u32 {
    entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .split('-')
        .collect::<Vec<_>>()[0]
        .parse::<u32>()
        .unwrap()
}
