use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use musiqlibrary;

use super::super::gmodel;
use super::super::repl;

pub fn update_manual_track_mapping(
    raw_library: &musiqlibrary::RawLibrary,
    manual_ignore_albums: &BTreeSet<(String, String)>,
    mut not_initially_found: Vec<gmodel::CleanedLineItem>,
) -> (
    BTreeMap<gmodel::GPlayMusicKey, (musiqlibrary::FullTrackMetadata, u32)>,
    Vec<gmodel::CleanedLineItem>,
) {
    let manual_mapping_file_reader =
        fs::File::open("gplaymusic/intermediate/manual_track_mapping.json").unwrap();

    let existing_manual_mapping_vec: Vec<(
        gmodel::GPlayMusicKey,
        (musiqlibrary::FullTrackMetadata, u32),
    )> = serde_json::from_reader(manual_mapping_file_reader).unwrap();

    let existing_manual_mapping = existing_manual_mapping_vec.into_iter().collect();

    not_initially_found.sort_by_key(|t| t.play_count);

    let (manual_mapping, not_found) = repl::prompt_user_for_track_manual_mappings(
        &raw_library,
        existing_manual_mapping,
        &manual_ignore_albums,
        not_initially_found,
    );

    (manual_mapping, not_found)
}
