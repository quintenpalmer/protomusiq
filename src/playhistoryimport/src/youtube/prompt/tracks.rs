use std::collections::BTreeMap;

use super::intermediate;

pub fn get_all_track_info_resolved(
    keyed_by_artist_sorted_by_max_song_play_count: &Vec<(String, Vec<(String, Vec<String>)>)>,
    raw_library: &musiqlibrary::RawLibrary,
    perfect_artist_map: &BTreeMap<String, String>,
    manual_artist_map: &BTreeMap<String, String>,
) -> BTreeMap<String, musiqlibrary::TrackUniqueIdentifier> {
    let mut manual_track_map = intermediate::load_intermediate_tracks();

    for (artist, track_watch_ats) in keyed_by_artist_sorted_by_max_song_play_count.iter() {
        for (track_name, watched_ats) in track_watch_ats.iter() {
            let matched_artist = match perfect_artist_map.get(artist) {
                Some(matched_a) => Some(matched_a),
                None => match manual_artist_map.get(artist) {
                    Some(matched_a) => Some(matched_a),
                    None => None,
                },
            };
            println!(
                "artist '{}' (matched as {:?}) has track '{}' with {} views",
                artist,
                matched_artist,
                track_name,
                watched_ats.len()
            );
            match matched_artist {
                Some(artist) => {
                    println!("found an actual artist, will only search their songs");
                }
                None => {
                    println!("could not find an actual artist, will search all songs");
                }
            }
        }
    }

    intermediate::save_intermediate_tracks(&manual_track_map);

    manual_track_map
}
