use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use musiqlibrary;

use super::super::{smodel, repl};

pub fn update_manual_track_mapping(
    raw_library: &musiqlibrary::RawLibrary,
    manual_ignore_albums: &BTreeSet<(String, String)>,
    not_initially_found: BTreeMap<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>,
) -> (
    BTreeMap<smodel::SpotifyKey, (musiqlibrary::FullTrackMetadata, Vec<smodel::CleanedLineItem>)>,
    BTreeMap<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>,
) {
    let manual_mapping_file_reader =
        fs::File::open("spotify/intermediate/manual_track_mapping.json").unwrap();

    let existing_manual_mapping_vec: Vec<(
        smodel::SpotifyKey,
        (musiqlibrary::FullTrackMetadata, Vec<smodel::CleanedLineItem>),
    )> = serde_json::from_reader(manual_mapping_file_reader).unwrap();

    let existing_manual_mapping = existing_manual_mapping_vec.into_iter().collect();

    let mut not_initially_found_vec: Vec<_> = not_initially_found.into_iter().collect();

    not_initially_found_vec.sort_by_key(|t| t.1.len());
    not_initially_found_vec.reverse();

    let (manual_mapping, not_found) = repl::prompt_user_for_track_manual_mappings(
        &raw_library,
        existing_manual_mapping,
        &manual_ignore_albums,
        not_initially_found_vec,
    );

    (manual_mapping, not_found)
}
