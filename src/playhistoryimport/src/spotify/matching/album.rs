use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;

use musiqlibrary;

use super::super::{repl, smodel, util};

pub fn update_album_mapping(
    raw_library: &musiqlibrary::RawLibrary,
    not_initially_found: &BTreeMap<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>,
) -> (
    BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    BTreeSet<(String, String)>,
) {
    // Existing Mapping File
    let manual_mapping_file_reader =
        fs::File::open("spotify/intermediate/manual_album_mapping.json").unwrap();

    let existing_manual_mapping_vec: Vec<((String, String), musiqlibrary::ArtistAlbumInfo)> =
        serde_json::from_reader(io::BufReader::new(manual_mapping_file_reader)).unwrap();

    let existing_album_manual_mapping = existing_manual_mapping_vec.into_iter().collect();

    // Existing Mapping File
    let ignore_file_reader = fs::File::open("spotify/intermediate/ignore_albums.json").unwrap();

    let existing_ignore_vec: Vec<(String, String)> =
        serde_json::from_reader(io::BufReader::new(ignore_file_reader)).unwrap();

    let existing_ignore_albums = existing_ignore_vec.into_iter().collect();

    let artist_album_track_vecs = util::compute_track_artist_album_track_map(&not_initially_found);

    let (album_mapping, ignore_artist_albums) = repl::prompt_user_for_artist_album_manual_mappings(
        &raw_library,
        existing_album_manual_mapping,
        existing_ignore_albums,
        artist_album_track_vecs,
    );

    (album_mapping, ignore_artist_albums)
}
