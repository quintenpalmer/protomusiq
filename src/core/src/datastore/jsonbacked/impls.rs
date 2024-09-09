use crate::datastore;
use crate::datastore::jsonbacked::tracker;

impl datastore::traits::LiveReadOnlyTrackCountReporter for tracker::ReadOnlyTracker {
    fn get_live_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.get_track_count(id)
    }
}
