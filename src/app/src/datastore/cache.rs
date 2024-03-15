use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path;

use crate::model;

use super::localfs;

/// Build the cache/musicbrainz/artists dir
pub fn musicbrainz_artist_cache_dir() -> path::PathBuf {
    let app_data_path = model::functions::get_default_data_path();

    localfs::build_tree_for_dirs(&app_data_path, vec!["cache", "musicbrainz", "artists"])
}

/// Check if a raw.json file exists for an artist
pub fn raw_exists(musicbrainz_artist_cache: &path::PathBuf, artist_name: String) -> bool {
    let this_artist_cache_file = localfs::build_tree_for_file(
        &musicbrainz_artist_cache,
        vec![artist_name, "raw.json".to_string()],
    );

    this_artist_cache_file.exists()
}

/// Check if a match.json file exists for an artist
pub fn match_exists(musicbrainz_artist_cache: &path::PathBuf, artist_name: String) -> bool {
    let this_artist_cache_file = localfs::build_tree_for_file(
        &musicbrainz_artist_cache,
        vec![artist_name, "match.json".to_string()],
    );

    this_artist_cache_file.exists()
}

/// Read the raw.json file for an artist
pub fn read_musicbrainz_artist_cache_file(
    musicbrainz_artist_cache: &path::PathBuf,
    artist_name: String,
) -> String {
    let this_artist_cache_file = localfs::build_tree_for_file(
        &musicbrainz_artist_cache,
        vec![artist_name, "raw.json".to_string()],
    );

    let mut file = fs::File::open(this_artist_cache_file).unwrap();
    let mut ret = String::new();
    file.read_to_string(&mut ret).unwrap();
    ret
}

/// Read to the raw.json file for an artist with a given (assumed) json payload
pub fn write_musicbrainz_artist_cache_file(
    musicbrainz_artist_cache: &path::PathBuf,
    artist_name: String,
    contents: String,
) {
    let this_artist_cache_file = localfs::build_tree_for_file(
        &musicbrainz_artist_cache,
        vec![artist_name, "raw.json".to_string()],
    );

    let mut file = fs::File::create(this_artist_cache_file).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

/// Read the raw.json file for an artist
pub fn read_musicbrainz_artist_match_file(
    musicbrainz_artist_cache: &path::PathBuf,
    artist_name: String,
) -> Option<musicbrainz::Artist> {
    let this_artist_cache_file = localfs::build_tree_for_file(
        &musicbrainz_artist_cache,
        vec![artist_name, "match.json".to_string()],
    );

    serde_json::from_reader(io::BufReader::new(
        fs::File::open(this_artist_cache_file.clone()).ok()?,
    ))
    .ok()
}

/// Read to the raw.json file for an artist with a given (assumed) json payload
pub fn write_musicbrainz_artist_match_file(
    musicbrainz_artist_cache: &path::PathBuf,
    artist_name: String,
    artist: &musicbrainz::Artist,
) {
    let this_artist_cache_file = localfs::build_tree_for_file(
        &musicbrainz_artist_cache,
        vec![artist_name, "match.json".to_string()],
    );

    serde_json::to_writer_pretty(
        io::BufWriter::new(fs::File::create(this_artist_cache_file.clone()).unwrap()),
        artist,
    )
    .unwrap();
}
