use std::path::PathBuf;

use chrono::{DateTime, Local};

use musiqlibrary;

use crate::datastore::traits::LiveHistoryReadDS;

use super::JSONTracker;

pub struct ReadOnlyTracker {
    inner: JSONTracker,
}

impl ReadOnlyTracker {
    pub fn new(
        app_data_path: &PathBuf,
        hostname: String,
        allowed_tracker_files: &Option<Vec<PathBuf>>,
    ) -> Self {
        ReadOnlyTracker {
            inner: JSONTracker::new(app_data_path, hostname, allowed_tracker_files),
        }
    }

    pub fn get_track_count(&self, track_identifier: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.inner.get_track_count(track_identifier)
    }

    pub fn get_track_history(
        &self,
        track_identifier: &musiqlibrary::TrackUniqueIdentifier,
    ) -> Option<&Vec<DateTime<Local>>> {
        self.inner.get_track_history(track_identifier)
    }
}
