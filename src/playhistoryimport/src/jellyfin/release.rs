use std::fs;
use std::io;

use super::jmodel::ResultingInformation;
use crate::model::MusiqHistoricalPlayHistoryTuple;

pub fn compute_release_files(
    resulting_information: &ResultingInformation,
) -> Vec<MusiqHistoricalPlayHistoryTuple> {
    let manually_mapped = &resulting_information.manual_mapping;
    let automatically_mapped = &resulting_information.matched_tracks_json_ready;

    let mut jellyfin_ready: Vec<MusiqHistoricalPlayHistoryTuple> = Vec::new();

    for (track, (_key, play_count)) in automatically_mapped.iter() {
        let entry = MusiqHistoricalPlayHistoryTuple {
            track: track.clone(),
            play_count: *play_count,
        };
        jellyfin_ready.push(entry);
    }

    for (_key, (track, play_count)) in manually_mapped.iter() {
        let entry = MusiqHistoricalPlayHistoryTuple {
            track: track.clone(),
            play_count: *play_count,
        };
        jellyfin_ready.push(entry);
    }

    jellyfin_ready
}

pub fn write_release_files(release_ready_play_history: &Vec<MusiqHistoricalPlayHistoryTuple>) {
    println!(
        "INFO:\twriting {} length file of release ready jellyfin play history",
        release_ready_play_history.len()
    );
    let release_file =
        fs::File::create("jellyfin/output/release/jellyfin_musiq_play_history.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(release_file),
        &release_ready_play_history,
    )
    .unwrap();
}
