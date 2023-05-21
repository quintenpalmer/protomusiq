use std::io;

use musiqlibrary;

use crate::model;
use crate::util::{config, logging};

use crate::datastore::jsonbacked::{self, tracker};
use crate::datastore::loader;
use crate::datastore::sqlitebacked;

#[derive(Debug)]
pub enum Error {}

pub fn entry_point() -> Result<(), Error> {
    let config_state = config::get_default_config();

    let mut keep_going = true;

    let stdin = io::stdin(); // We get `Stdin` here.

    while keep_going {
        let mut buffer = String::new();
        println!("what should we do?");
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        match buffer {
            "exit" | "quit" => {
                println!("exiting");
                keep_going = false;
            }
            "create_tables" => {
                println!("creating tables");
                create_tables(&config_state)
            }
            "seed_data" => {
                println!("seeding data");
                seed(&config_state);
            }
            "read_library" => {
                println!("reading all data");
                read(&config_state);
            }
            "check_tracked_entries" => {
                println!("checking tracked entries");
                check_tracked(&config_state);
            }
            "produce_id" => {
                println!("let's produce an id");
                produce_id()
            }
            _ => {
                println!("unknown command '{}'", buffer);
            }
        };
    }

    Ok(())
}

fn read(config_state: &model::AppConfigState) {
    let conn = sqlitebacked::Connections::first_bootup(config_state.clone());

    let mut logger = logging::Logger::new(logging::LogType::Timing, "sql load");

    logger.print_elapsed("starting loading (should be 0)");

    let library = conn.get_library();

    logger.print_elapsed("got library from sql");

    let mut count = 0;
    for artist in library.artists.values() {
        for album in artist.albums.values() {
            for disc in album.discs.values() {
                for _track in disc.tracks.values() {
                    count += 1;
                }
            }
        }
    }
    println!("I see this many tracks: {}", count);

    logger.print_elapsed("counting tracks");
}

fn create_tables(config_state: &model::AppConfigState) {
    sqlitebacked::Connections::first_bootup(config_state.clone());
}

fn seed(_config_state: &model::AppConfigState) {
    // TODO use bootstrap_tracks and the rest
}

fn check_tracked(config_state: &model::AppConfigState) {
    let library = jsonbacked::tracklibrary::load_library_from_cache_and_scan(
        &config_state,
        &loader::Loader::NoCache,
    );
    let prehistory_records = tracker::list_all_tracker_records(&config_state.app_data_path);

    let mut no_matches = Vec::new();

    for (track_id, _date_times) in prehistory_records.iter() {
        match library.artists.get(&track_id.artist_id) {
            None => no_matches.push((track_id.clone(), None, None)),
            Some(artist) => match artist.albums.get(&track_id.album_id) {
                None => no_matches.push((
                    track_id.clone(),
                    Some(artist.artist_info.artist_name.clone()),
                    None,
                )),
                Some(album) => match album.discs.get(&track_id.disc_no) {
                    None => no_matches.push((
                        track_id.clone(),
                        Some(artist.artist_info.artist_name.clone()),
                        Some(album.album_info.album_name.clone()),
                    )),
                    Some(disc) => match disc.tracks.get(&track_id.track_no) {
                        None => no_matches.push((
                            track_id.clone(),
                            Some(artist.artist_info.artist_name.clone()),
                            Some(album.album_info.album_name.clone()),
                        )),
                        Some(_track) => (),
                    },
                },
            },
        }
    }

    for line in no_matches.iter() {
        println!("{:?}", line);
    }
}

fn produce_id() {
    let stdin = io::stdin(); // We get `Stdin` here.

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().to_string();
    let id = musiqlibrary::ID::new(&buffer);
    println!("{:?}", id);
}
