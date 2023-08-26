mod output;
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

    let manual_track_map = prompt::tracks::get_all_track_info_resolved(
        &keyed_by_artist_sorted_by_max_song_play_count,
        &raw_library,
        &perfect_artist_map,
        &manual_artist_map,
    );

    output::write_release_output(
        &raw_library,
        &keyed_by_artist_sorted_by_max_song_play_count,
        &manual_track_map,
    )
}
