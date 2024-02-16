use std::fs;
use std::io;

use crate::model;

use super::smodel;

pub fn compute_and_write_release_info(debug_info: &smodel::DebugInfo) {
    let musiq_ready = compute_release_files(&debug_info);
    write_release_files(&musiq_ready);
}

fn compute_release_files(
    resulting_information: &smodel::DebugInfo,
) -> Vec<model::MusiqHistoricalPlayHistoryTuple> {
    let manually_mapped = &resulting_information.manual_track_mapping;
    let automatically_mapped = &resulting_information.found_keys_in_library_matches;

    let mut gplaymsuic_musiq_ready: Vec<model::MusiqHistoricalPlayHistoryTuple> = Vec::new();

    for (_key, (track, plays)) in automatically_mapped.iter() {
        let entry = model::MusiqHistoricalPlayHistoryTuple {
            track: track.clone(),
            play_count: plays.len() as u32,
        };
        gplaymsuic_musiq_ready.push(entry);
    }

    for (_key, (track, plays)) in manually_mapped.iter() {
        let entry = model::MusiqHistoricalPlayHistoryTuple {
            track: track.clone(),
            play_count: plays.len() as u32,
        };
        gplaymsuic_musiq_ready.push(entry);
    }

    gplaymsuic_musiq_ready
}

fn write_release_files(release_ready_play_history: &Vec<model::MusiqHistoricalPlayHistoryTuple>) {
    println!(
        "INFO:\twriting {} length file of release ready spotify play history",
        release_ready_play_history.len()
    );
    let release_file =
        fs::File::create("spotify/output/release/spotify_musiq_play_history.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(release_file),
        &release_ready_play_history,
    )
    .unwrap();
}
