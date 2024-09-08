use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path;

use super::super::{smodel, util};

use super::{album, tracks};

pub fn match_spotify_music_data_for_musiqapp() -> smodel::DebugInfo {
    let all_spotify_music_tracks_file = fs::File::open("spotify/input/play_history.json").unwrap();

    let clean_line_items: Vec<smodel::CleanedLineItem> =
        serde_json::from_reader(io::BufReader::new(all_spotify_music_tracks_file)).unwrap();

    let play_info_line_items: BTreeMap<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>> =
        clean_line_items
            .into_iter()
            .fold(BTreeMap::new(), |mut total, current| {
                let structured = total.entry(current.get_key()).or_insert(Vec::new());
                structured.push(current);
                total
            });
    println!(
        "found this many keys {} into line items (will occur multiple times)",
        play_info_line_items.len()
    );

    let library_path = "/home/quinten/storage/media/music/bestexisting";
    let lib_path = path::PathBuf::from(&library_path);

    let tracks = musiqlibrary::find_files(&lib_path).unwrap();

    let mut tracks_as_line_items: BTreeMap<
        smodel::SpotifyKey,
        (
            musiqlibrary::FullTrackMetadata,
            Vec<smodel::CleanedLineItem>,
        ),
    > = tracks.iter().fold(BTreeMap::new(), |mut total, current| {
        total
            .entry(util::generate_spotify_key(&current))
            .or_insert((current.clone(), Vec::new()));
        total
    });

    let keyed_library_items = tracks_as_line_items
        .iter()
        .map(|(key, (val, _))| (key.clone(), val.clone()))
        .collect();

    let raw_library =
        musiqlibrary::RawLibrary::from_track_list(Some(&library_path), tracks).unwrap();

    let mut not_initially_found =
        BTreeMap::<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>::new();

    for (key, line_item) in play_info_line_items.iter() {
        match tracks_as_line_items.get_mut(key) {
            Some(value) => {
                if value.1.len() == 0 {
                    value.1.append(&mut line_item.clone());
                } else {
                    panic!("found dupe: {:?}", key)
                }
            }
            None => {
                not_initially_found.insert(key.clone(), line_item.clone());
                ()
            }
        }
    }
    println!(
        "could not initially find any matches for: {}",
        not_initially_found.len()
    );

    let (manual_album_mapping, manual_ignore_albums) =
        album::update_album_mapping(&raw_library, &not_initially_found);
    println!(
        "{} album matches manually found",
        manual_album_mapping.len()
    );

    let mut not_second_found = BTreeMap::<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>::new();

    let mut duplicates_found = Vec::new();

    for (maybe_now_found_key, plays) in not_initially_found.iter() {
        let not_found_key = maybe_now_found_key.get_key_with_mappings(&manual_album_mapping);
        match tracks_as_line_items.get_mut(&not_found_key) {
            Some(value) => {
                if value.1.len() == 0 {
                    value.1.append(&mut plays.clone());
                } else {
                    duplicates_found.push(not_found_key.clone());
                }
            }
            None => {
                not_second_found.insert(maybe_now_found_key.clone(), plays.clone());
                ()
            }
        }
    }
    println!("found these duplicates:");
    for dup in duplicates_found.iter() {
        println!("{:?}", dup);
    }

    let (manual_track_mapping, not_found) =
        tracks::update_manual_track_mapping(&raw_library, &manual_ignore_albums, not_second_found);

    let mut found_keys_in_library_matches: BTreeMap<
        smodel::SpotifyKey,
        (
            musiqlibrary::FullTrackMetadata,
            Vec<smodel::CleanedLineItem>,
        ),
    > = BTreeMap::new();

    for (key, line_item) in tracks_as_line_items.iter() {
        match line_item.1.len() {
            0 => (),
            _ => {
                found_keys_in_library_matches.insert(key.clone(), line_item.clone());
                ()
            }
        }
    }

    println!(
        "found this many matches in the library: {}",
        found_keys_in_library_matches.len()
    );

    let mut play_info_lines_vec: Vec<(smodel::SpotifyKey, Vec<smodel::CleanedLineItem>)> =
        play_info_line_items.into_iter().collect();

    play_info_lines_vec.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));

    let play_info_lines_count: Vec<(smodel::SpotifyKey, usize)> = play_info_lines_vec
        .into_iter()
        .map(|(a, b)| (a, b.len()))
        .collect();

    let mut not_found_albums = BTreeMap::new();

    for (key, _) in not_found.iter() {
        let entry = not_found_albums
            .entry((
                key.master_metadata_album_artist_name.clone(),
                key.master_metadata_album_album_name.clone(),
            ))
            .or_insert(0);
        *entry += 1;
    }

    smodel::DebugInfo {
        play_info_lines_count,
        not_found,
        not_found_albums,
        keyed_library_items,
        manual_track_mapping,
        manual_album_mapping,
        manual_ignore_albums,
        found_keys_in_library_matches,
    }
}
