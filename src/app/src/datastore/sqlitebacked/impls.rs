use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use crate::datastore;

use super::concrete::{conn, dbmodel::bridge, query};

impl bridge::SixtyFourLibrary {
    fn new(conn: &conn::Connections) -> Self {
        let artists = query::artists::select_artists(&conn.shared_db);
        let albums = query::albums::select_albums(&conn.shared_db);
        let discs = query::discs::select_discs(&conn.shared_db);
        let tracks = query::tracks::select_tracks(&conn.shared_db);

        let bridge = bridge::SixtyFourLibrary::from_db_rows(artists, albums, discs, tracks);

        bridge
    }
}

struct CachingConn {
    conn: conn::Connections,
    bridge: bridge::SixtyFourLibrary,
}

impl CachingConn {
    #[allow(unused)]
    pub fn new(conn: conn::Connections) -> Self {
        let bridge = bridge::SixtyFourLibrary::new(&conn);

        CachingConn {
            conn: conn,
            bridge: bridge,
        }
    }
}

impl datastore::traits::LiveReadOnlyTrackCountReporter for CachingConn {
    fn get_live_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.conn.get_live_track_count(id, &self.bridge)
    }
}

impl datastore::traits::HistoricalTrackCountReporter for CachingConn {
    fn get_historical_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        self.conn.get_historical_track_count(id, &self.bridge)
    }
}

pub struct SqlitePreHistoryReporter {
    bridge: bridge::SixtyFourLibrary,
    prehistorical_track_id_to_count: BTreeMap<u32, usize>,
}

impl SqlitePreHistoryReporter {
    pub fn new(conn: conn::Connections) -> Self {
        let bridge = bridge::SixtyFourLibrary::new(&conn);
        let prehistorical_track_id_to_count = conn.get_all_historical_track_counts();

        SqlitePreHistoryReporter {
            bridge: bridge,
            prehistorical_track_id_to_count: prehistorical_track_id_to_count,
        }
    }
}

impl datastore::traits::HistoricalTrackCountReporter for SqlitePreHistoryReporter {
    fn get_historical_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        let db_track = self.bridge.track_from_unique_key(id);
        let count = self
            .prehistorical_track_id_to_count
            .get(&db_track.id)
            .unwrap_or(&0);
        *count
    }
}

pub struct SqliteLiveHistoryReporter {
    bridge: bridge::SixtyFourLibrary,
    livehistory_track_id_to_count: BTreeMap<u32, usize>,
}

impl SqliteLiveHistoryReporter {
    pub fn new(conn: conn::Connections) -> Self {
        let bridge = bridge::SixtyFourLibrary::new(&conn);
        let livehistory_track_id_to_count = conn.get_all_live_track_counts();

        SqliteLiveHistoryReporter {
            bridge: bridge,
            livehistory_track_id_to_count: livehistory_track_id_to_count,
        }
    }
}

impl datastore::traits::LiveReadOnlyTrackCountReporter for SqliteLiveHistoryReporter {
    fn get_live_track_count(&self, id: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        let db_track = self.bridge.track_from_unique_key(id);
        let count = self
            .livehistory_track_id_to_count
            .get(&db_track.id)
            .unwrap_or(&0);
        *count
    }
}

pub struct SqliteLiveHistoryRecorder {
    conn: conn::Connections,
    bridge: bridge::SixtyFourLibrary,
}

impl SqliteLiveHistoryRecorder {
    pub fn new(conn: conn::Connections) -> Self {
        let bridge = bridge::SixtyFourLibrary::new(&conn);

        SqliteLiveHistoryRecorder {
            conn: conn,
            bridge: bridge,
        }
    }
}

impl datastore::traits::LiveHistoryWriteDS for SqliteLiveHistoryRecorder {
    fn increment_track(&mut self, track: &musiqlibrary::FullTrackMetadata) {
        self.conn
            .increment_track_with_date(&self.bridge, track, Local::now())
    }

    fn increment_track_with_date(
        &mut self,
        track: &musiqlibrary::FullTrackMetadata,
        date_time: DateTime<Local>,
    ) {
        self.conn
            .increment_track_with_date(&self.bridge, track, date_time)
    }

    fn increment_tracks_with_dates(
        &mut self,
        tracks_with_dates: Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)>,
    ) {
        self.conn
            .increment_tracks_with_dates(&self.bridge, tracks_with_dates)
    }
}

#[allow(dead_code)]
pub struct SqliteAlbumArtRecorder {
    conn: conn::Connections,
    bridge: bridge::SixtyFourLibrary,
}

impl datastore::traits::CachedAlbumImageInfo for SqliteAlbumArtRecorder {
    fn get_all_known_art(
        &self,
        _albums: Vec<musiqlibrary::AlbumInfo>,
    ) -> BTreeMap<datastore::traits::AlbumArtKey, Vec<u8>> {
        BTreeMap::new()
    }

    fn write_all_art(&mut self, _art: BTreeMap<datastore::traits::AlbumArtKey, Vec<u8>>) {}
}
