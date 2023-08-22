use std::collections::BTreeMap;
use std::fs;

use serde_json;

pub fn load_intermediate_artists() -> BTreeMap<String, String> {
    let manual_mapping_file_reader =
        fs::File::open("youtube/intermediate/manual_artist_mapping.json").unwrap();

    let existing_manual_mapping: BTreeMap<String, String> =
        serde_json::from_reader(manual_mapping_file_reader).unwrap();

    existing_manual_mapping
}

pub fn save_intermediate_artists(artist_map: &BTreeMap<String, String>) {
    let manual_mapping_file_file =
        fs::File::create("youtube/output/debug/1_manual_artist_mapping.json").unwrap();
    serde_json::to_writer_pretty(manual_mapping_file_file, artist_map).unwrap();
}

pub fn load_intermediate_tracks() -> BTreeMap<String, musiqlibrary::TrackUniqueIdentifier> {
    let manual_mapping_file_reader =
        fs::File::open("youtube/intermediate/manual_track_mapping.json").unwrap();

    let existing_manual_mapping: BTreeMap<String, musiqlibrary::TrackUniqueIdentifier> =
        serde_json::from_reader(manual_mapping_file_reader).unwrap();

    existing_manual_mapping
}
