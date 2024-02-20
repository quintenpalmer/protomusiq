use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::datastore;
use crate::datastore::localfs;

use super::super::common;

pub struct JSONTracker {
    pub tracker_db_json_path: PathBuf,
    pub local_tracks: BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>>,
    pub all_tracks: BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>>,
}

impl JSONTracker {
    pub fn new(
        app_data_path: &PathBuf,
        hostname: String,
        allowed_tracker_files: &Option<Vec<PathBuf>>,
    ) -> Self {
        let (raw_tracker, final_path): (RawTrackedPayload, PathBuf) = common::bootstrap_raw_data(
            app_data_path,
            vec!["data", "tracker", format!("{}.json", hostname).as_str()],
        );

        let all_tracks = list_all_tracker_records(app_data_path, allowed_tracker_files);

        JSONTracker {
            tracker_db_json_path: final_path,
            local_tracks: raw_tracker.to_btree_map(),
            all_tracks: all_tracks,
        }
    }
}

impl datastore::traits::LiveHistoryReadDS for JSONTracker {
    fn get_track_history(
        &self,
        track_identifier: &musiqlibrary::TrackUniqueIdentifier,
    ) -> Option<&Vec<DateTime<Local>>> {
        self.all_tracks.get(track_identifier)
    }

    fn get_track_count(&self, track_identifier: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        match self.all_tracks.get(track_identifier) {
            Some(v) => {
                //println!("I found a value for {:?}: {}", track_identifier, v.len());
                v.len()
            }
            None => 0,
        }
    }
}

impl datastore::traits::LiveHistoryWriteDS for JSONTracker {
    fn increment_track(&mut self, track: &musiqlibrary::FullTrackMetadata) {
        self.increment_track_with_date(track, Local::now())
    }

    fn increment_track_with_date(
        &mut self,
        track: &musiqlibrary::FullTrackMetadata,
        date_time: DateTime<Local>,
    ) {
        let count = self
            .local_tracks
            .entry(musiqlibrary::TrackUniqueIdentifier::from_track(track))
            .or_insert(Vec::new());
        count.push(date_time);

        let raw_tracker = RawTrackedPayload::from_btree_map(&self.local_tracks);

        serde_json::to_writer(
            io::BufWriter::new(fs::File::create(&self.tracker_db_json_path).unwrap()),
            &raw_tracker,
        )
        .unwrap();
    }

    fn increment_tracks_with_dates(
        &mut self,
        tracks_with_dates: Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)>,
    ) {
        for (track, date_time) in tracks_with_dates.into_iter() {
            let count = self
                .local_tracks
                .entry(musiqlibrary::TrackUniqueIdentifier::from_track(&track))
                .or_insert(Vec::new());
            count.push(date_time);
        }

        let raw_tracker = RawTrackedPayload::from_btree_map(&self.local_tracks);

        serde_json::to_writer(
            io::BufWriter::new(fs::File::create(&self.tracker_db_json_path).unwrap()),
            &raw_tracker,
        )
        .unwrap();
    }
}

pub fn list_all_tracker_records(
    app_data_path: &PathBuf,
    allowed_tracker_files: &Option<Vec<PathBuf>>,
) -> BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>> {
    let allowed_files = common::FileAllower::new(allowed_tracker_files);

    let mut all_tracks = BTreeMap::new();

    let tracker_files_dir = localfs::build_tree_for_dirs(app_data_path, vec!["data", "tracker"]);

    for tracker_file in fs::read_dir(tracker_files_dir).unwrap() {
        let tracker_file = tracker_file.unwrap().path();
        println!("tracker_file: {:?}", tracker_file);
        if allowed_files.is_allowed(&tracker_file) {
            match tracker_file.extension().map(|x| x.to_str()) {
                Some(Some("json")) => {
                    let current_tracks: RawTrackedPayload =
                        common::maybe_get_existing_raw_data(&tracker_file).unwrap();
                    for (current_track, mut current_track_count) in
                        current_tracks.tracks.into_iter()
                    {
                        all_tracks
                            .entry(current_track)
                            .or_insert(Vec::new())
                            .append(&mut current_track_count);
                    }
                }
                Some(Some(ext)) => println!("skipping non json file ({}) in data/tracker/", ext),
                Some(None) => println!("skipping non json file in data/tracker/"),
                None => println!("skipping non json file in data/tracker/"),
            }
        } else {
            println!("skipping file which was not allowed from config");
        }
    }
    all_tracks
}

#[derive(Deserialize, Serialize, Default)]
pub struct RawTrackedPayload {
    pub tracks: Vec<(musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>)>,
}

impl RawTrackedPayload {
    pub fn to_btree_map(
        self,
    ) -> BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>> {
        self.tracks.into_iter().collect()
    }

    pub fn from_btree_map(
        tracks: &BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>>,
    ) -> Self {
        RawTrackedPayload {
            tracks: tracks
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        }
    }
}
