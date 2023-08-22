mod parse;
mod prompt;
mod sort;
mod util;
mod ytmodel;

pub fn translate_youtube_play_history() {
    let entries = util::get_entries();

    let raw_library = util::get_library();

    let keyed_by_artist_sorted_by_max_song_play_count =
        sort::sort_entries_by_song_max_play_count(entries);

    let (perfect_artist_map, manual_artist_map) = prompt::artists::get_all_artist_info_resolved(
        &keyed_by_artist_sorted_by_max_song_play_count,
        &raw_library,
    );

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
        }
    }
}
