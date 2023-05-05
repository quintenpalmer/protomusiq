use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use musiqlibrary;

use super::super::{gmodel, repl, util};

pub fn update_album_mapping(
    raw_library: &musiqlibrary::RawLibrary,
    manual_artist_mapping: &BTreeMap<String, musiqlibrary::ArtistInfo>,
    not_initially_found: &Vec<gmodel::CleanedLineItem>,
) -> (
    BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    BTreeSet<(String, String)>,
) {
    // Existing Mapping File
    let manual_mapping_file_reader =
        fs::File::open("gplaymusic/intermediate/manual_album_mapping.json").unwrap();

    let existing_manual_mapping_vec: Vec<((String, String), musiqlibrary::ArtistAlbumInfo)> =
        serde_json::from_reader(manual_mapping_file_reader).unwrap();

    let existing_album_manual_mapping = existing_manual_mapping_vec.into_iter().collect();

    // Existing Mapping File
    let ignore_file_reader = fs::File::open("gplaymusic/intermediate/ignore_albums.json").unwrap();

    let existing_ignore_vec: Vec<(String, String)> =
        serde_json::from_reader(ignore_file_reader).unwrap();

    let existing_ignore_albums = existing_ignore_vec.into_iter().collect();

    let artist_album_track_vecs = util::compute_track_artist_album_track_map(&not_initially_found);

    let (album_mapping, ignore_artist_albums) = repl::prompt_user_for_artist_album_manual_mappings(
        &raw_library,
        existing_album_manual_mapping,
        existing_ignore_albums,
        &manual_artist_mapping,
        artist_album_track_vecs,
    );

    (album_mapping, ignore_artist_albums)
}
