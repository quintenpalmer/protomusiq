use musiqlibrary;

use crate::datastore;

pub type AugmentedLibrary = musiqlibrary::Library<AugmentedTrack>;

pub fn augmented_from_raw(
    raw_library: musiqlibrary::RawLibrary,
    tracked_data: Box<dyn datastore::traits::LiveReadOnlyTrackCountReporter>,
    historical_data: Box<dyn datastore::traits::HistoricalTrackCountReporter>,
) -> AugmentedLibrary {
    raw_library.map_into(&|track| {
        let uniq_track_id = musiqlibrary::TrackUniqueIdentifier::from_track(&track);
        let live_play_count = tracked_data.get_live_track_count(&uniq_track_id);
        let historical_play_count = historical_data.get_historical_track_count(&uniq_track_id);
        let total_play_count = live_play_count + historical_play_count;

        AugmentedTrack {
            augmented: AugmentedData {
                play_count: total_play_count,
                tagged_genres: Vec::new(),
            },
            metadata: track,
        }
    })
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AugmentedTrack {
    pub metadata: musiqlibrary::FullTrackMetadata,
    pub augmented: AugmentedData,
}

impl AugmentedTrack {
    pub fn played_seconds(&self) -> u64 {
        self.metadata.duration.as_secs() * (self.augmented.play_count as u64)
    }
}

impl Into<musiqlibrary::FullTrackMetadata> for AugmentedTrack {
    fn into(self) -> musiqlibrary::FullTrackMetadata {
        self.metadata
    }
}

pub fn album_total_play_count(album: &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>) -> usize {
    let mut total = 0;
    for (_, disc) in album.discs.iter() {
        for (_, track) in disc.tracks.iter() {
            total += track.augmented.play_count;
        }
    }
    total
}

pub fn artist_total_play_count(artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>) -> usize {
    let mut total = 0;
    for (_, album) in artist.albums.iter() {
        total += album_total_play_count(album);
    }
    total
}

pub fn album_total_played_duration(album: &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>) -> u64 {
    let mut total = 0;
    for (_, disc) in album.discs.iter() {
        for (_, track) in disc.tracks.iter() {
            total += track.played_seconds();
        }
    }
    total
}

pub fn artist_total_played_duration(
    artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>,
) -> u64 {
    let mut total = 0;
    for (_, album) in artist.albums.iter() {
        total += album_total_played_duration(album);
    }
    total
}

pub fn album_track_duration_total(album: &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>) -> u64 {
    let mut total = 0;
    for (_, disc) in album.discs.iter() {
        for (_, track) in disc.tracks.iter() {
            total += track.metadata.duration.as_secs();
        }
    }
    total
}

pub fn artist_track_duration_total(
    artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>,
) -> u64 {
    let mut total = 0;
    for (_, album) in artist.albums.iter() {
        total += album_track_duration_total(album);
    }
    total
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AugmentedData {
    pub play_count: usize,
    pub tagged_genres: Vec<String>,
}
