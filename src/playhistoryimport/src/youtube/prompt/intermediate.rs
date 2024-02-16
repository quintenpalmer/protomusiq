use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;

use serde_json;

pub fn load_intermediate_artists() -> BTreeMap<String, String> {
    let manual_mapping_file_reader =
        fs::File::open("youtube/intermediate/manual_artist_mapping.json").unwrap();

    let existing_manual_mapping: BTreeMap<String, String> =
        serde_json::from_reader(io::BufReader::new(manual_mapping_file_reader)).unwrap();

    existing_manual_mapping
}

pub fn save_intermediate_artists(artist_map: &BTreeMap<String, String>) {
    let manual_mapping_file_file =
        fs::File::create("youtube/output/debug/1_manual_artist_mapping.json").unwrap();
    serde_json::to_writer_pretty(io::BufWriter::new(manual_mapping_file_file), artist_map).unwrap();
}

pub fn load_intermediate_tracks() -> BTreeMap<String, musiqlibrary::TrackUniqueIdentifier> {
    let manual_mapping_file_reader =
        fs::File::open("youtube/intermediate/manual_track_mapping.json").unwrap();

    let existing_manual_mapping: BTreeMap<String, musiqlibrary::TrackUniqueIdentifier> =
        serde_json::from_reader(io::BufReader::new(manual_mapping_file_reader)).unwrap();

    existing_manual_mapping
}

pub fn save_intermediate_tracks(track_map: &BTreeMap<String, musiqlibrary::TrackUniqueIdentifier>) {
    let manual_mapping_file_file =
        fs::File::create("youtube/output/debug/2_manual_track_mapping.json").unwrap();
    serde_json::to_writer_pretty(io::BufWriter::new(manual_mapping_file_file), track_map).unwrap();
}

pub fn load_intermediate_ignore_artists() -> BTreeSet<String> {
    let manual_mapping_file_reader =
        fs::File::open("youtube/intermediate/ignore_artist_mapping.json").unwrap();

    let existing_manual_mapping: BTreeSet<String> =
        serde_json::from_reader(io::BufReader::new(manual_mapping_file_reader)).unwrap();

    existing_manual_mapping
}

pub fn save_intermediate_ignore_artists(artist_map: &BTreeSet<String>) {
    let manual_mapping_file_file =
        fs::File::create("youtube/output/debug/3_ignore_artist_mapping.json").unwrap();
    serde_json::to_writer_pretty(io::BufWriter::new(manual_mapping_file_file), artist_map).unwrap();
}
