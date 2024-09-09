use musiqcore::datastore;

use super::prehistory;

impl datastore::traits::HistoricalTrackCountReporter for prehistory::Reporter {
    fn get_historical_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.get_track_count(id)
    }
}
