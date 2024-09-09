use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;

use chrono::{DateTime, Local};

use crate::datastore;
use crate::datastore::localfs;

use musiqcore::model::jsonbacked::tracker::RawTrackedPayload;

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
            all_tracks,
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

    let child_paths = fs::read_dir(tracker_files_dir.clone())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    println!("with these many tracker files: {}", child_paths.len());
    for tracker_file in child_paths {
        println!("this tracker file: {:?}", tracker_file);
    }

    let num_threads = std::thread::available_parallelism()
        .map(|x| x.into())
        .unwrap_or(1);

    println!("creating thread pool with {} threads", num_threads);

    let tpool = threadpool::ThreadPool::new(num_threads);
    let (tx, rx) = mpsc::channel();

    let mut total_to_read = 0;

    for tracker_file in fs::read_dir(tracker_files_dir).unwrap() {
        let tx = tx.clone();

        let tracker_file = tracker_file.unwrap().path();
        let should_read = allowed_files.is_allowed(&tracker_file)
            && match tracker_file.extension().map(|x| x.to_str()) {
                Some(Some("json")) => true,
                Some(Some(ext)) => {
                    println!("skipping non json file ({}) in data/tracker/", ext);
                    false
                }
                Some(None) => {
                    println!("skipping non json file in data/tracker/");
                    false
                }
                None => {
                    println!("skipping non json file in data/tracker/");
                    false
                }
            };

        if should_read {
            total_to_read += 1;
        }

        tpool.execute(move || {
            println!("tracker_file: {:?}", tracker_file);
            if should_read {
                println!("reading json file ({:?}) in data/tracker/", tracker_file);
                let current_tracks: RawTrackedPayload =
                    common::maybe_get_existing_raw_data(&tracker_file).unwrap();
                for (current_track, current_track_count) in current_tracks.tracks.into_iter() {
                    tx.send((current_track, current_track_count))
                        .expect("please let the recv be listening");
                }
            } else {
                println!("skipping file which was not allowed from config");
            }
        });
    }

    drop(tx);

    println!("total to read will be: {}", total_to_read);

    for (current_track, mut current_track_count) in rx {
        all_tracks
            .entry(current_track)
            .or_insert(Vec::new())
            .append(&mut current_track_count);
    }
    all_tracks
}
