use std::collections::BTreeMap;
use std::fs;

use crate::model;

pub fn write_release_output(
    raw_library: &musiqlibrary::RawLibrary,
    keyed_by_artist_sorted_by_max_song_play_count: &Vec<(String, Vec<(String, Vec<String>)>)>,
    manual_track_map: &BTreeMap<String, musiqlibrary::TrackUniqueIdentifier>,
) {
    let release_vec = compute_release_output(
        raw_library,
        keyed_by_artist_sorted_by_max_song_play_count,
        manual_track_map,
    );
    write_release_files(&release_vec);
}

fn compute_release_output(
    raw_library: &musiqlibrary::RawLibrary,
    keyed_by_artist_sorted_by_max_song_play_count: &Vec<(String, Vec<(String, Vec<String>)>)>,
    manual_track_map: &BTreeMap<String, musiqlibrary::TrackUniqueIdentifier>,
) -> Vec<model::MusiqHistoricalPlayHistoryTuple> {
    let mut ret = Vec::new();

    for (_artist, track_watch_vec) in keyed_by_artist_sorted_by_max_song_play_count.iter() {
        for (track_name, watch_vec) in track_watch_vec.iter() {
            match manual_track_map.get(track_name) {
                Some(track_unique_id) => {
                    let track = raw_library.get_track(track_unique_id);
                    let release_entry = model::MusiqHistoricalPlayHistoryTuple {
                        track: track.clone(),
                        play_count: watch_vec.len() as u32,
                    };
                    ret.push(release_entry);
                }
                None => println!(
                    "skipping track we didn't manually map: {} (with {} views)",
                    track_name,
                    watch_vec.len()
                ),
            }
        }
    }

    return ret;
}

fn write_release_files(release_ready_play_history: &Vec<model::MusiqHistoricalPlayHistoryTuple>) {
    println!(
        "INFO:\twriting {} length file of release ready youtube play history",
        release_ready_play_history.len()
    );
    let release_file =
        fs::File::create("youtube/output/release/youtube_musiq_play_history.json").unwrap();
    serde_json::to_writer_pretty(release_file, &release_ready_play_history).unwrap();
}
