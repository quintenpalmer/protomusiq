use std::env;
use std::path;
use std::time;

use super::augmented;

pub fn get_default_config_path() -> path::PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let config_path = path::Path::new(&home_dir)
        .join(".config")
        .join("musiqapp")
        .join("config.json");

    config_path
}

pub fn get_default_data_path() -> path::PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let data_path = path::Path::new(&home_dir)
        .join(".local")
        .join("share")
        .join("musiq")
        .join("v1");

    data_path
}

pub fn tracks_after_including(
    tracks: &Vec<augmented::AugmentedTrack>,
    current_track: &augmented::AugmentedTrack,
) -> Vec<augmented::AugmentedTrack> {
    let mut ret = Vec::new();
    let mut found = false;
    for iter_track in tracks.iter() {
        if !found && iter_track == current_track {
            found = true;
        }
        if found {
            ret.push(iter_track.clone());
        }
    }

    ret
}

pub fn compute_track_list_duration(
    tracks: &Vec<musiqlibrary::FullTrackMetadata>,
) -> time::Duration {
    let mut duration = time::Duration::new(0, 0);
    for track in tracks.iter() {
        duration += track.duration;
    }
    duration
}
