use std::io;
use std::path;
use std::thread;
use std::time;

use crate::model;
use crate::util::{config, logging};

use crate::datastore::cache;
use crate::datastore::jsonbacked::{self, tracker};
use crate::datastore::loader;
use crate::datastore::sqlitebacked;

#[derive(Debug)]
pub enum Error {}

pub fn entry_point() -> Result<(), Error> {
    println!("prototype start");
    let config_state = config::get_default_config();

    let lib_path = config_state.library_path.clone();
    println!("library path is: {:?}", lib_path);

    println!("getting library");
    let library = musiqlibrary::RawLibrary::new(lib_path.clone()).unwrap();
    println!("got library");

    let musicbrainz_artist_cache = cache::musicbrainz_artist_cache_dir();

    println!("iterating over artists");
    for artist in library.artists.values() {
        grab_raw_musicbrainz_data(&musicbrainz_artist_cache, &artist.artist_info);

        select_singular_artist_match(&musicbrainz_artist_cache, &artist.artist_info);
    }

    println!("going to try to reload all and compute lev distance");
    for ml_artist in library.artists.values() {
        match cache::read_musicbrainz_artist_match_file(
            &musicbrainz_artist_cache,
            ml_artist.artist_info.artist_name.clone(),
        ) {
            Some(artist) => {
                let distance = model::functions::levenshtein(
                    &artist.name.to_lowercase(),
                    &ml_artist.artist_info.artist_name.to_lowercase(),
                );
                println!(
                    "{}\t between \"{}\" and \"{}\"",
                    distance, artist.name, ml_artist.artist_info.artist_name
                );
            }
            None => {
                println!(
                    "1000\t between ??? and \"{}\"",
                    ml_artist.artist_info.artist_name
                );
            }
        }
    }

    Ok(())
}

pub fn grab_raw_musicbrainz_data(
    musicbrainz_artist_cache: &path::PathBuf,
    artist_info: &musiqlibrary::ArtistInfo,
) {
    println!(
        "checking if cached musicbrainz exists for {}",
        artist_info.artist_name,
    );
    if cache::raw_exists(&musicbrainz_artist_cache, artist_info.artist_name.clone()) {
        println!(
            "cached musicbrainz data does exist for {}",
            artist_info.artist_name,
        );
    } else {
        println!(
            "querying musicbrainz for artist: {}",
            artist_info.artist_name,
        );
        let json_str = musicbrainz::query_for_artist_raw(artist_info.artist_name.clone());

        println!("writing json for: {}", artist_info.artist_name);
        cache::write_musicbrainz_artist_cache_file(
            &musicbrainz_artist_cache,
            artist_info.artist_name.clone(),
            json_str,
        );

        println!(
            "done querying musicbrainz for artist: {}",
            artist_info.artist_name
        );

        thread::sleep(time::Duration::new(2, 0))
    }
}

pub fn select_singular_artist_match(
    musicbrainz_artist_cache: &path::PathBuf,
    artist_info: &musiqlibrary::ArtistInfo,
) {
    println!(
        "trying to find singular artist match: {}",
        artist_info.artist_name,
    );
    if cache::match_exists(&musicbrainz_artist_cache, artist_info.artist_name.clone()) {
        println!(
            "matched single musicbrainz entry does exist for artist: {}",
            artist_info.artist_name,
        );
    } else {
        println!(
            "finding single musicbrainz match for artist: {}",
            artist_info.artist_name,
        );
        let json_str = cache::read_musicbrainz_artist_cache_file(
            &musicbrainz_artist_cache,
            artist_info.artist_name.clone(),
        );

        let artist_results = musicbrainz::ArtistListResult::from_json(json_str);

        let mut found_100_matches = Vec::new();

        for artist in artist_results.artists.into_iter() {
            if artist.score == 100 {
                found_100_matches.push(artist);
            }
        }

        match found_100_matches.as_slice() {
            &[ref single_match] => cache::write_musicbrainz_artist_match_file(
                &musicbrainz_artist_cache,
                artist_info.artist_name.clone(),
                single_match,
            ),
            &[] => println!(
                "found no 100 matches for this artist: {}",
                artist_info.artist_name
            ),
            _ => println!(
                "found multiple 100 matches for this artist: {}",
                artist_info.artist_name
            ),
        };

        println!(
            "done finding single artist match for: {}",
            artist_info.artist_name
        );
    }
}

#[allow(unused)]
pub fn sql_mode() -> Result<(), Error> {
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

fn read(config_state: &model::app::AppConfigState) {
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

fn create_tables(config_state: &model::app::AppConfigState) {
    sqlitebacked::Connections::first_bootup(config_state.clone());
}

fn seed(_config_state: &model::app::AppConfigState) {
    // TODO use bootstrap_tracks and the rest
}

fn check_tracked(config_state: &model::app::AppConfigState) {
    let library = jsonbacked::tracklibrary::load_library_from_cache_and_scan(
        config_state,
        &loader::Loader::NoCache,
    );
    let prehistory_records = tracker::list_all_tracker_records(
        &config_state.app_data_path,
        &config_state.allowed_tracker_files,
    );

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
