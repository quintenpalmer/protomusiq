use std::collections::BTreeMap;

use super::smodel;

pub fn generate_spotify_key(track: &musiqlibrary::FullTrackMetadata) -> smodel::SpotifyKey {
    smodel::SpotifyKey {
        master_metadata_track_name: track.title.to_lowercase(),
        master_metadata_album_artist_name: track.album_artist.to_lowercase(),
        master_metadata_album_album_name: track.album.to_lowercase(),
    }
}

pub fn compute_track_artist_album_track_map(
    not_initially_found: &BTreeMap<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>,
) -> Vec<((String, String), Vec<smodel::CleanedLineItem>)> {
    let mut artist_album_track_map: BTreeMap<(String, String), Vec<smodel::CleanedLineItem>> =
        BTreeMap::new();

    for (not_found, vec) in not_initially_found.iter() {
        let track_list = artist_album_track_map
            .entry((
                not_found.master_metadata_album_artist_name.to_lowercase(),
                not_found.master_metadata_album_album_name.to_lowercase(),
            ))
            .or_insert(Vec::new());
        track_list.append(&mut vec.clone());
    }

    let mut artist_album_track_vecs: Vec<((String, String), Vec<smodel::CleanedLineItem>)> =
        artist_album_track_map
            .into_iter()
            .map(|(key, val)| (key, val.into_iter().collect()))
            .collect();

    artist_album_track_vecs.sort_by(|a, b| {
        smodel::compute_album_play_count(&a.1).cmp(&smodel::compute_album_play_count(&b.1))
    });
    artist_album_track_vecs.reverse();

    artist_album_track_vecs
}
