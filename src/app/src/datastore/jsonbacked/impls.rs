use super::prehistory;
use super::tracker;

use super::super::super::datastore;

impl datastore::traits::HistoricalTrackCountReporter for prehistory::Reporter {
    fn get_historical_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.get_track_count(id)
    }
}

impl datastore::traits::LiveReadOnlyTrackCountReporter for tracker::ReadOnlyTracker {
    fn get_live_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.get_track_count(id)
    }
}
