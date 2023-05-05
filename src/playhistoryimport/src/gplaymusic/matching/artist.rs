use std::collections::BTreeMap;
use std::fs;

use musiqlibrary;

use super::super::{gmodel, repl, util};

pub fn update_artist_mapping(
    raw_library: &musiqlibrary::RawLibrary,
    not_initially_found: &Vec<gmodel::CleanedLineItem>,
) -> BTreeMap<String, musiqlibrary::ArtistInfo> {
    let album_manual_mapping_file_reader =
        fs::File::open("gplaymusic/intermediate/manual_artist_mapping.json").unwrap();

    let existing_artist_manual_mapping: BTreeMap<String, musiqlibrary::ArtistInfo> =
        serde_json::from_reader(album_manual_mapping_file_reader).unwrap();

    let artist_album_track_vecs = util::compute_track_artist_album_track_map(&not_initially_found);

    let artist_mapping = repl::prompt_user_for_artist_manual_mappings(
        &raw_library,
        existing_artist_manual_mapping,
        artist_album_track_vecs,
    );

    artist_mapping
}
