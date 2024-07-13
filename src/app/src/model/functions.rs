use std::time;

use super::augmented;

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
