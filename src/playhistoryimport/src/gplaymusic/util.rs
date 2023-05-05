use std::collections::BTreeMap;

use musiqlibrary;

use super::gmodel;

pub fn generate_gplaymusic_key(track: &musiqlibrary::FullTrackMetadata) -> gmodel::GPlayMusicKey {
    gmodel::GPlayMusicKey {
        artist: track.album_artist.to_lowercase(),
        album: track.album.to_lowercase(),
        title: track.title.to_lowercase(),
    }
}

pub fn compute_track_artist_album_track_map(
    not_initially_found: &Vec<gmodel::CleanedLineItem>,
) -> Vec<(String, Vec<(String, Vec<gmodel::CleanedLineItem>)>)> {
    let mut artist_album_track_map: BTreeMap<
        String,
        BTreeMap<String, Vec<gmodel::CleanedLineItem>>,
    > = BTreeMap::new();

    for not_found in not_initially_found.iter() {
        let album_map = artist_album_track_map
            .entry(not_found.artist.to_lowercase())
            .or_insert(BTreeMap::new());
        let track_list = album_map
            .entry(not_found.album.to_lowercase())
            .or_insert(Vec::new());
        track_list.push(not_found.clone());
    }

    let mut artist_album_track_vecs: Vec<(String, Vec<(String, Vec<gmodel::CleanedLineItem>)>)> =
        artist_album_track_map
            .into_iter()
            .map(|(key, val)| (key, val.into_iter().collect()))
            .collect();

    artist_album_track_vecs.sort_by(|a, b| {
        gmodel::compute_artist_play_count(&a.1).cmp(&gmodel::compute_artist_play_count(&b.1))
    });
    artist_album_track_vecs.reverse();

    artist_album_track_vecs
}
