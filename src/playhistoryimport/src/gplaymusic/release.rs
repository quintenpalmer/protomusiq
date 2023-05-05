use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use super::gmodel::BestEffortMatchedInformation;
use crate::model::MusiqHistoricalPlayHistoryTuple;

pub fn compute_release_files(
    resulting_information: &BestEffortMatchedInformation,
) -> Vec<MusiqHistoricalPlayHistoryTuple> {
    let old_manually_mapped = &resulting_information.manual_track_mapping;
    let mut manually_mapped = BTreeMap::new();
    let automatically_mapped = &resulting_information.matched_tracks_json_ready;

    let mut seen_map = BTreeSet::new();

    for (track, (_key, _play_count)) in automatically_mapped.iter() {
        seen_map.insert(musiqlibrary::TrackUniqueIdentifier::from_track(&track));
    }

    for (key, (track, play_count)) in old_manually_mapped.iter() {
        if seen_map.contains(&musiqlibrary::TrackUniqueIdentifier::from_track(&track)) {
            println!(
                "would be skipping this old manual mapped entry: {:?} - {} / {}",
                key, track.title, play_count
            );
        }
        manually_mapped.insert(key.clone(), (track.clone(), play_count.clone()));
    }

    let mut gplaymsuic_musiq_ready: Vec<MusiqHistoricalPlayHistoryTuple> = Vec::new();

    for (track, (_key, play_count)) in automatically_mapped.iter() {
        let entry = MusiqHistoricalPlayHistoryTuple {
            track: track.clone(),
            play_count: *play_count,
        };
        gplaymsuic_musiq_ready.push(entry);
    }

    for (_key, (track, play_count)) in manually_mapped.iter() {
        let entry = MusiqHistoricalPlayHistoryTuple {
            track: track.clone(),
            play_count: *play_count,
        };
        gplaymsuic_musiq_ready.push(entry);
    }

    gplaymsuic_musiq_ready
}

pub fn write_release_files(release_ready_play_history: &Vec<MusiqHistoricalPlayHistoryTuple>) {
    println!(
        "INFO:\twriting {} length file of release ready gplaymusic play history",
        release_ready_play_history.len()
    );
    let release_file =
        fs::File::create("gplaymusic/output/release/gplaymusic_musiq_play_history.json").unwrap();
    serde_json::to_writer_pretty(release_file, &release_ready_play_history).unwrap();
}
