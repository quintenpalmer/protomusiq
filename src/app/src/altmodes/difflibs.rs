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
    input_btree_left: &BTreeMap<
        musiqlibrary::TrackUniqueIdentifier,
        musiqlibrary::FullTrackMetadata,
    >,
    input_btree_right: &BTreeMap<
        musiqlibrary::TrackUniqueIdentifier,
        musiqlibrary::FullTrackMetadata,
    >,
) -> (
    Vec<musiqlibrary::FullTrackMetadata>,
    Vec<musiqlibrary::FullTrackMetadata>,
) {
    let input_left: BTreeMap<PartialTrackMetadata, musiqlibrary::FullTrackMetadata> =
        input_btree_left
            .values()
            .map(|v| (PartialTrackMetadata::from_full(v), v.clone()))
            .collect();
    let input_right: BTreeMap<PartialTrackMetadata, musiqlibrary::FullTrackMetadata> =
        input_btree_right
            .values()
            .map(|v| (PartialTrackMetadata::from_full(v), v.clone()))
            .collect();

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

/// Full Track Metadata
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct PartialTrackMetadata {
    pub title: String,
    pub track: u64,
    pub raw_track: Option<u64>,
    pub disc: u64,
    pub raw_disc: Option<u64>,
    pub disc_total: Option<u64>,
    pub album: String,
    pub raw_album: Option<String>,
    pub album_id: musiqlibrary::ID,
    pub album_artist: String,
    pub album_artist_id: musiqlibrary::ID,
    pub track_artist: String,
    pub track_artist_id: musiqlibrary::ID,
    pub genre: String,
    pub date_number: u32,
    pub raw_date: String,
}

impl PartialTrackMetadata {
    fn from_full(full: &musiqlibrary::FullTrackMetadata) -> Self {
        PartialTrackMetadata {
            title: full.title.clone(),
            track: full.track,
            raw_track: full.raw_track,
            disc: full.disc,
            raw_disc: full.raw_disc,
            disc_total: full.disc_total,
            album: full.album.clone(),
            raw_album: full.raw_album.clone(),
            album_id: full.album_id,
            album_artist: full.album_artist.clone(),
            album_artist_id: full.album_artist_id,
            track_artist: full.track_artist.clone(),
            track_artist_id: full.track_artist_id,
            genre: full.genre.clone(),
            date_number: full.date_number,
            raw_date: full.raw_date.clone(),
        }
    }
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
