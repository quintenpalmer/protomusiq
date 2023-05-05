use serde::{Deserialize, Serialize};

use musiqlibrary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusiqHistoricalPlayHistoryTuple {
    pub track: musiqlibrary::FullTrackMetadata,
    pub play_count: u32,
}

pub fn split_on_criteria<T, F: Fn(&T) -> bool>(vec: Vec<T>, function: F) -> (Vec<T>, Vec<T>) {
    let mut matches = Vec::new();
    let mut not_matches = Vec::new();

    for elem in vec.into_iter() {
        if function(&elem) {
            matches.push(elem);
        } else {
            not_matches.push(elem);
        }
    }

    (matches, not_matches)
}

pub fn exists_lowercase_artist(
    raw_library: &musiqlibrary::RawLibrary,
    artist_name: &String,
) -> bool {
    get_lowercase_artist(&raw_library, &artist_name).is_some()
}

pub fn get_lowercase_artist<'a>(
    raw_library: &'a musiqlibrary::RawLibrary,
    artist_name: &String,
) -> Option<&'a musiqlibrary::KeyedArtistAlbums<musiqlibrary::FullTrackMetadata>> {
    for (_artist_id, artist) in raw_library.artists.iter() {
        if artist.artist_info.artist_name.to_lowercase() == artist_name.to_lowercase() {
            return Some(artist);
        }
    }
    return None;
}

pub fn exists_lowercase_album(
    artist: &musiqlibrary::KeyedArtistAlbums<musiqlibrary::FullTrackMetadata>,
    album_name: &String,
) -> bool {
    for (_album_id, album) in artist.albums.iter() {
        if album.album_info.album_name.to_lowercase() == album_name.to_lowercase() {
            return true;
        }
    }
    return false;
}
