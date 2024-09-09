use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use crate::model;

pub trait HistoricalTrackCountReporter {
    fn get_historical_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize;
}

pub trait LiveReadOnlyTrackCountReporter {
    fn get_live_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize;
}

pub trait LiveHistoryReadDS {
    fn get_track_history(
        &self,
        track_identifier: &musiqlibrary::TrackUniqueIdentifier,
    ) -> Option<&Vec<DateTime<Local>>>;
    fn get_track_count(&self, track_identifier: &musiqlibrary::TrackUniqueIdentifier) -> usize;
}

pub trait LiveHistoryWriteDS {
    fn increment_track(&mut self, track: &musiqlibrary::FullTrackMetadata);
    fn increment_track_with_date(
        &mut self,
        track: &musiqlibrary::FullTrackMetadata,
        date_time: DateTime<Local>,
    );
    fn increment_tracks_with_dates(
        &mut self,
        tracks_with_dates: Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)>,
    );
}

#[allow(unused)]
pub struct AlbumArtKey {
    pub album_key: musiqlibrary::AlbumUniqueIdentifier,
    pub size: model::AlbumSizeWithOrig,
}

pub trait CachedAlbumImageInfo {
    #[allow(unused)]
    fn get_all_known_art(
        &self,
        albums: Vec<musiqlibrary::AlbumInfo>,
    ) -> BTreeMap<AlbumArtKey, Vec<u8>>;

    #[allow(unused)]
    fn write_all_art(&mut self, art: BTreeMap<AlbumArtKey, Vec<u8>>);
}
