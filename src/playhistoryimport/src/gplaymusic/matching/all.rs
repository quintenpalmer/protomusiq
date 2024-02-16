use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path;

use musiqlibrary;

use crate::model;

use super::super::{gmodel, util};

use super::{album, artist, track};

pub fn match_gplay_music_data_for_musiqapp() -> gmodel::BestEffortMatchedInformation {
    let all_gplay_music_tracks_file = fs::File::open("gplaymusic/input/all_tracks.json").unwrap();

    let raw_line_items: Vec<gmodel::CleanedLineItem> =
        serde_json::from_reader(io::BufReader::new(all_gplay_music_tracks_file)).unwrap();

    let (all_zero_line_items, non_zero_line_items) =
        model::split_on_criteria(raw_line_items, |x| x.play_count == 0);

    let play_info_line_items: BTreeMap<gmodel::GPlayMusicKey, gmodel::CleanedLineItem> =
        non_zero_line_items
            .into_iter()
            .map(|x| (x.get_key(), x.clone()))
            .collect();

    let library_path = "/home/quinten/storage/media/music/bestexisting";
    let lib_path = path::PathBuf::from(&library_path);

    let tracks = musiqlibrary::find_files(&lib_path, &lib_path).unwrap();

    let mut tracks_as_line_items: BTreeMap<
        gmodel::GPlayMusicKey,
        (musiqlibrary::FullTrackMetadata, u32),
    > = tracks
        .iter()
        .map(|track| (util::generate_gplaymusic_key(&track), (track.clone(), 0)))
        .collect();

    let raw_library =
        musiqlibrary::RawLibrary::from_track_list(Some(&library_path), tracks).unwrap();

    let mut not_initially_found = Vec::<gmodel::CleanedLineItem>::new();

    for (key, line_item) in play_info_line_items.iter() {
        match tracks_as_line_items.get_mut(key) {
            Some(value) => {
                if value.1 == 0 {
                    value.1 += line_item.play_count
                } else {
                    panic!("found dupe: {:?}", key)
                }
            }
            None => not_initially_found.push(line_item.clone()),
        }
    }

    let manual_artist_mapping = artist::update_artist_mapping(&raw_library, &not_initially_found);
    println!(
        "{} artist matches manually found",
        manual_artist_mapping.len()
    );

    let (manual_album_mapping, manual_ignore_albums) =
        album::update_album_mapping(&raw_library, &manual_artist_mapping, &not_initially_found);
    println!(
        "{} album matches manually found",
        manual_album_mapping.len()
    );

    let mut not_second_found = Vec::<gmodel::CleanedLineItem>::new();

    let mut duplicates_found = Vec::new();

    for maybe_now_found in not_initially_found.iter() {
        let not_found_key =
            maybe_now_found.get_key_with_mappings(&manual_artist_mapping, &manual_album_mapping);
        match tracks_as_line_items.get_mut(&not_found_key) {
            Some(value) => {
                if value.1 == 0 {
                    value.1 += maybe_now_found.play_count
                } else {
                    duplicates_found.push(not_found_key.clone());
                }
            }
            None => not_second_found.push(maybe_now_found.clone()),
        }
    }
    println!("found these duplicates:");
    for dup in duplicates_found.iter() {
        println!("{:?}", dup);
    }

    let (manual_track_mapping, not_found) =
        track::update_manual_track_mapping(&raw_library, &manual_ignore_albums, not_second_found);

    let mut full_library_with_matches: Vec<(
        musiqlibrary::FullTrackMetadata,
        (gmodel::GPlayMusicKey, u32),
    )> = Vec::new();

    for (key, (full, play_count)) in tracks_as_line_items.into_iter() {
        full_library_with_matches.push((full, (key, play_count)));
    }

    let (existing_library_with_zero_new_count, matched_tracks_json_ready) =
        model::split_on_criteria(full_library_with_matches, |(_full, (_key, play_count))| {
            *play_count == 0
        });

    let ignore_album_info_with_play_counts =
        sort_ignore_album_info_with_play_counts(&manual_ignore_albums, &play_info_line_items);

    let resulting_information = gmodel::BestEffortMatchedInformation {
        all_zero_line_items,
        not_found,
        existing_library_with_zero_new_count,
        manual_track_mapping,
        manual_artist_mapping,
        manual_album_mapping,
        manual_ignore_albums,
        ignore_album_info_with_play_counts,
        matched_tracks_json_ready,
    };

    resulting_information
}

fn sort_ignore_album_info_with_play_counts(
    manual_ignore_albums: &BTreeSet<(String, String)>,
    play_info_line_items: &BTreeMap<gmodel::GPlayMusicKey, gmodel::CleanedLineItem>,
) -> BTreeMap<(String, String), u32> {
    let mut ret = BTreeMap::new();

    for (key, value) in play_info_line_items.iter() {
        if manual_ignore_albums.contains(&(key.artist.clone(), key.album.clone())) {
            let value_at = ret
                .entry((key.artist.clone(), key.album.clone()))
                .or_insert(0);
            *value_at += value.play_count;
        }
    }

    ret
}

pub fn _lowercase_library(raw_library: musiqlibrary::RawLibrary) -> musiqlibrary::RawLibrary {
    let artists = raw_library
        .artists
        .into_iter()
        .map(|(_artist_id, mut artist)| {
            let artist_id = musiqlibrary::ID::new(&artist.artist_info.artist_name.to_lowercase());
            let albums = artist
                .albums
                .into_iter()
                .map(|(_album_id, album)| {
                    let album_id =
                        musiqlibrary::ID::new(&album.album_info.album_name.to_lowercase());
                    (album_id, album)
                })
                .collect();
            artist.albums = albums;
            (artist_id, artist)
        })
        .collect();
    musiqlibrary::Library {
        scan_prefix: raw_library.scan_prefix,
        artists: artists,
    }
}
