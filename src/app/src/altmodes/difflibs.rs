use std::collections::BTreeMap;

use crate::util::config;

#[derive(Debug)]
pub enum Error {}

pub fn entry_point() -> Result<(), Error> {
    eprintln!("let's compute some diffs");
    let config_state = config::get_default_config();

    eprintln!("getting (un)compressed library paths");
    let uncompressed_library_path = config_state.library_path.clone();
    let compressed_library_path = config_state.compressed_library_path.unwrap().clone();

    eprintln!("getting (un)compressed libraries");
    let uncompressed_library = musiqlibrary::RawLibrary::new(uncompressed_library_path).unwrap();
    let compressed_library = musiqlibrary::RawLibrary::new(compressed_library_path).unwrap();

    eprintln!("converting (un)compressed libraries to payloads");
    let uncompressed_track_map = track_uniq_tree(uncompressed_library);
    let compressed_track_map = track_uniq_tree(compressed_library);

    eprintln!("computing diff between uncompressed and compressed libraries");
    let (only_uncompressed, only_compressed) =
        compute_diff(&uncompressed_track_map, &compressed_track_map);

    eprintln!("only uncompressed:");
    eprintln!("{}", only_uncompressed.len());

    for track in only_uncompressed.iter() {
        println!(
            "ffmpeg {}",
            track
                .path
                .clone()
                .into_os_string()
                .to_string_lossy()
                .to_string(),
        );
    }

    eprintln!("only compressed:");
    eprintln!("{}", only_compressed.len());

    println!("");
    for track in only_compressed.iter() {
        println!(
            "rm {}",
            track
                .path
                .clone()
                .into_os_string()
                .to_string_lossy()
                .to_string(),
        );
    }

    Ok(())
}

fn compute_diff(
    input_left: &BTreeMap<musiqlibrary::TrackUniqueIdentifier, musiqlibrary::FullTrackMetadata>,
    input_right: &BTreeMap<musiqlibrary::TrackUniqueIdentifier, musiqlibrary::FullTrackMetadata>,
) -> (
    Vec<musiqlibrary::FullTrackMetadata>,
    Vec<musiqlibrary::FullTrackMetadata>,
) {
    let mut only_left = Vec::new();
    let mut only_right = Vec::new();

    for (left_key, left_val) in input_left.iter() {
        if !input_right.contains_key(left_key) {
            only_left.push(left_val.clone());
        }
    }

    for (right_key, right_val) in input_right.iter() {
        if !input_left.contains_key(right_key) {
            only_right.push(right_val.clone());
        }
    }

    (only_left, only_right)
}

fn track_uniq_tree(
    library: musiqlibrary::RawLibrary,
) -> BTreeMap<musiqlibrary::TrackUniqueIdentifier, musiqlibrary::FullTrackMetadata> {
    let mut ret_tree = BTreeMap::new();

    for artist in library.artists.values() {
        for album in artist.albums.values() {
            for disc in album.discs.values() {
                for track in disc.tracks.values() {
                    ret_tree.insert(track.to_unique_id(), track.clone());
                }
            }
        }
    }

    ret_tree
}
