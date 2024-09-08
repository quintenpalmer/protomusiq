use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path;

use serde_json;

use musiqlibrary;

use super::jmodel::{ResultingInformation, UserDataPlayInfo};
use crate::model;

use super::repl;

pub fn compute_jellyfin_play_history_for_musiqapp() -> ResultingInformation {
    let jellyfin_json_file_reader = fs::File::open("jellyfin/input/jellyfin.json").unwrap();

    let raw_line_items: Vec<UserDataPlayInfo> =
        serde_json::from_reader(io::BufReader::new(jellyfin_json_file_reader)).unwrap();

    let (all_zero_line_items, non_zero_line_items) =
        model::split_on_criteria(raw_line_items, |x| x.play_count == 0);

    let (uuid_line_items, non_uuid_line_items) =
        model::split_on_criteria(non_zero_line_items, is_expected_uuid_pattern);

    let play_info_line_items: BTreeMap<String, UserDataPlayInfo> = non_uuid_line_items
        .into_iter()
        .map(|x| (x.key.to_lowercase(), x.clone()))
        .collect();

    let library_path = "/home/quinten/storage/media/music/bestexisting";
    let lib_path = path::PathBuf::from(&library_path);

    let tracks = musiqlibrary::find_files(&lib_path).unwrap();

    let mut tracks_as_line_items: BTreeMap<String, (musiqlibrary::FullTrackMetadata, u32)> = tracks
        .iter()
        .map(|track| (generate_jellyfin_key(&track), (track.clone(), 0)))
        .collect();

    let raw_library =
        musiqlibrary::RawLibrary::from_track_list(Some(&library_path), tracks).unwrap();

    let mut not_initially_found = Vec::<UserDataPlayInfo>::new();

    for (key, line_item) in play_info_line_items.iter() {
        match tracks_as_line_items.get_mut(key) {
            Some(value) => {
                if value.1 == 0 {
                    value.1 += line_item.play_count
                } else {
                    panic!("found dupe: {}", key)
                }
            }
            None => not_initially_found.push(line_item.clone()),
        }
    }

    let manual_mapping_file_reader =
        fs::File::open("jellyfin/intermediate/manual_mapping.json").unwrap();

    let existing_manual_mapping: BTreeMap<String, (musiqlibrary::FullTrackMetadata, u32)> =
        serde_json::from_reader(io::BufReader::new(manual_mapping_file_reader)).unwrap();

    let (manual_mapping, not_found) = repl::prompt_user_for_manual_mappings(
        &raw_library,
        existing_manual_mapping,
        not_initially_found,
    );

    let mut full_library_with_matches: Vec<(musiqlibrary::FullTrackMetadata, (String, u32))> =
        Vec::new();

    for (key, (full, play_count)) in tracks_as_line_items.into_iter() {
        full_library_with_matches.push((full, (key, play_count)));
    }

    let (existing_library_with_zero_new_count, matched_tracks_json_ready) =
        model::split_on_criteria(full_library_with_matches, |(_full, (_key, play_count))| {
            *play_count == 0
        });

    let resulting_information = ResultingInformation {
        all_zero_line_items,
        uuid_line_items,
        existing_library_with_zero_new_count,
        not_found,
        manual_mapping,
        matched_tracks_json_ready,
    };

    resulting_information
}

fn generate_jellyfin_key(track: &musiqlibrary::FullTrackMetadata) -> String {
    let album_info = match track.raw_album {
        Some(ref a) => format!("{}-", a),
        None => "".to_string(),
    };
    let disc_info = match track.raw_disc {
        Some(ref i) => format!("{:04}-", i),
        None => "".to_string(),
    };
    let track_info = match track.raw_track {
        Some(ref i) => format!("{:04}", i),
        None => "".to_string(),
    };
    format!(
        "{}-{}{}{}{}",
        track.album_artist, album_info, disc_info, track_info, track.title
    )
    .to_lowercase()
}

fn is_expected_uuid_pattern(item: &UserDataPlayInfo) -> bool {
    let pieces: Vec<&str> = item.key.split('-').collect();
    return pieces.len() == 5
        && (is_uuid_with_len(vec![8], pieces[0])
            && is_uuid_with_len(vec![4], pieces[1])
            && is_uuid_with_len(vec![4], pieces[2])
            && is_uuid_with_len(vec![4], pieces[3])
            && is_uuid_with_len(vec![12, 18], pieces[4]));
}

fn is_uuid_with_len(lens: Vec<usize>, piece: &str) -> bool {
    if lens.contains(&piece.len()) {
        return piece.chars().all(|x| x.is_ascii_alphanumeric());
    } else {
        return false;
    }
}
