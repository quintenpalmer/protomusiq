use std::sync::mpsc;
use std::thread;

use chrono::{DateTime, Local};

use musiqcore::datastore;
use musiqcore::datastore::jsonbacked::tracker;

use crate::datastore::loader;
use crate::datastore::sqlitebacked;
use crate::shared;

pub fn create_backend_with_client(
    config_state: musiqcore::model::app::AppConfigState,
    loader: loader::Loader,
) -> shared::Client<shared::TrackerMessage> {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    thread::spawn(move || run_forever(config_state, recv_for_backend, loader));

    shared::Client::new(sender_for_client)
}

struct ForkWriter {
    json: tracker::JSONTracker,
    sqlite: sqlitebacked::SqliteLiveHistoryRecorder,
}

impl datastore::traits::LiveHistoryWriteDS for ForkWriter {
    fn increment_track(&mut self, track: &musiqlibrary::FullTrackMetadata) {
        self.json.increment_track(track);
        self.sqlite.increment_track(track);
    }

    fn increment_track_with_date(
        &mut self,
        track: &musiqlibrary::FullTrackMetadata,
        date_time: DateTime<Local>,
    ) {
        self.json.increment_track_with_date(track, date_time);
        self.sqlite.increment_track_with_date(track, date_time);
    }

    fn increment_tracks_with_dates(
        &mut self,
        tracks_with_dates: Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)>,
    ) {
        self.json
            .increment_tracks_with_dates(tracks_with_dates.clone());
        self.sqlite.increment_tracks_with_dates(tracks_with_dates);
    }
}

pub fn run_forever(
    config_state: musiqcore::model::app::AppConfigState,
    rx: mpsc::Receiver<shared::TrackerMessage>,
    loader: loader::Loader,
) {
    let mut tracker: Box<dyn datastore::traits::LiveHistoryWriteDS> = match loader {
        loader::Loader::NoCache | loader::Loader::Json => {
            let json_tracker = tracker::JSONTracker::new(
                &config_state.app_data_path.to_path_buf(),
                config_state.hostname.clone(),
                &config_state.allowed_tracker_files,
            );
            Box::new(json_tracker)
        }
        loader::Loader::Sqlite(conn) => {
            let sqlite_tracker = sqlitebacked::SqliteLiveHistoryRecorder::new(conn);
            Box::new(sqlite_tracker)
        }
        loader::Loader::Latest(conn) => {
            let json_tracker = tracker::JSONTracker::new(
                &config_state.app_data_path.to_path_buf(),
                config_state.hostname.clone(),
                &config_state.allowed_tracker_files,
            );

            let sqlite_tracker = sqlitebacked::SqliteLiveHistoryRecorder::new(conn);

            Box::new(ForkWriter {
                json: json_tracker,
                sqlite: sqlite_tracker,
            })
        }
    };

    loop {
        match rx.recv() {
            Ok(msg) => match msg {
                shared::TrackerMessage::SongStarted(track) => {
                    println!("we're playing: {}", track.metadata.title);
                    tracker.increment_track(&track.metadata);
                }
            },
            Err(_e) => {
                println!("recv sees that all clients have closed");
                break;
            }
        }
    }
}
