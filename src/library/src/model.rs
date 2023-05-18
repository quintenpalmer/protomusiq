use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path;
use std::time;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Conflicts(Vec<FullTrackMetadata>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ID {
    inner: u64,
}

impl ID {
    pub fn new<T: Hash>(t: &T) -> Self {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        ID { inner: s.finish() }
    }

    pub fn from_u64(input: u64) -> Self {
        ID { inner: input }
    }

    pub fn hashed(&self) -> u64 {
        self.inner
    }
}

/// Full Track Metadata
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FullTrackMetadata {
    pub title: String,
    pub track: u64,
    pub raw_track: Option<u64>,
    pub disc: u64,
    pub raw_disc: Option<u64>,
    pub disc_total: Option<u64>,
    pub album: String,
    pub raw_album: Option<String>,
    pub album_id: ID,
    pub album_artist: String,
    pub album_artist_id: ID,
    pub track_artist: String,
    pub track_artist_id: ID,
    pub genre: String,
    pub date_number: u32,
    pub raw_date: String,
    pub duration: time::Duration,
    pub path: path::PathBuf,
    pub relative_path: path::PathBuf,
    pub last_modified: time::SystemTime,
    pub ext: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TrackPathInfo {
    pub path: path::PathBuf,
    pub relative_path: path::PathBuf,
    pub last_modified: time::SystemTime,
}

/// Opinionated Library Structure
pub type RawLibrary = Library<FullTrackMetadata>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library<T> {
    pub scan_prefix: Option<path::PathBuf>,
    pub artists: BTreeMap<ID, KeyedArtistAlbums<T>>,
}

impl<T> Library<T> {
    pub fn map_into<F: Fn(T) -> N, N>(self, function: &F) -> Library<N> {
        Library {
            scan_prefix: self.scan_prefix,
            artists: self
                .artists
                .into_iter()
                .map(|(key, value)| (key, value.map_into(&function)))
                .collect(),
        }
    }

    pub fn get_track(&self, key: &TrackUniqueIdentifier) -> &T {
        &self
            .artists
            .get(&key.artist_id)
            .unwrap()
            .albums
            .get(&key.album_id)
            .unwrap()
            .discs
            .get(&key.disc_no)
            .unwrap()
            .tracks
            .get(&key.track_no)
            .unwrap()
    }

    pub fn get_all_tracks(&self) -> Vec<&T> {
        let mut ret = Vec::new();
        for artist in self.artists.values() {
            ret.append(&mut artist.get_all_tracks());
        }
        ret
    }
}

/// Standalone Artist Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistInfo {
    pub artist_id: ID,
    pub artist_name: String,
}

/// Artist Info with Albums
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyedArtistAlbums<T> {
    pub artist_info: ArtistInfo,
    pub albums: BTreeMap<ID, KeyedAlbumTracks<T>>,
}

impl<T> KeyedArtistAlbums<T> {
    pub fn map_into<F: Fn(T) -> N, N>(self, function: &F) -> KeyedArtistAlbums<N> {
        KeyedArtistAlbums {
            artist_info: self.artist_info,
            albums: self
                .albums
                .into_iter()
                .map(|(key, value)| (key, value.map_into(&function)))
                .collect(),
        }
    }

    pub fn get_all_tracks(&self) -> Vec<&T> {
        let mut ret = Vec::new();
        for album in self.albums.values() {
            for disc in album.discs.values() {
                for track in disc.tracks.values() {
                    ret.push(track);
                }
            }
        }
        ret
    }

    pub fn album_count(&self) -> usize {
        self.albums.len()
    }

    pub fn track_count(&self) -> usize {
        let mut total = 0;
        for album in self.albums.values() {
            for disc in album.discs.values() {
                total += disc.tracks.len();
            }
        }
        total
    }
}

/// Standalone Album Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumInfo {
    pub album_id: ID,
    pub album_name: String,
    pub genres: BTreeSet<String>,
    pub total_duration: time::Duration,
    pub start_date: u32,
    pub end_date: u32,
    pub last_modified: time::SystemTime,
    pub path: path::PathBuf,
    pub relative_path: path::PathBuf,
}

/// Album Info with Discs (Tracks)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyedAlbumTracks<T> {
    pub album_info: AlbumInfo,
    pub discs: BTreeMap<u64, DiscTracks<T>>,
}

impl<T> KeyedAlbumTracks<T> {
    pub fn map_into<F: Fn(T) -> N, N>(self, function: &F) -> KeyedAlbumTracks<N> {
        KeyedAlbumTracks {
            album_info: self.album_info,
            discs: self
                .discs
                .into_iter()
                .map(|(key, value)| (key, value.map_into(&function)))
                .collect(),
        }
    }
}

impl<T> KeyedAlbumTracks<T>
where
    T: Into<FullTrackMetadata> + Clone,
{
    pub fn duration_seconds(&self) -> u64 {
        let mut total_duration = 0;
        for (_, disc) in self.discs.iter() {
            for (_, into_track) in disc.tracks.iter() {
                let track: FullTrackMetadata = into_track.clone().into();
                total_duration += track.duration.as_secs();
            }
        }
        total_duration
    }
}

/// Discs Tracks
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscTracks<T> {
    pub disc_no: u64,
    pub tracks: BTreeMap<u64, T>,
}

impl<T> DiscTracks<T> {
    pub fn map_into<F: Fn(T) -> N, N>(self, function: &F) -> DiscTracks<N> {
        DiscTracks {
            disc_no: self.disc_no,
            tracks: self
                .tracks
                .into_iter()
                .map(|(key, value)| (key, function(value)))
                .collect(),
        }
    }
}

/// Combination Artist and Album Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistAlbumInfo {
    pub artist: ArtistInfo,
    pub album: AlbumInfo,
}

/// Identifier for an Album
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AlbumUniqueIdentifier {
    pub artist_id: ID,
    pub album_id: ID,
}

impl AlbumUniqueIdentifier {
    pub fn new(artist_id: ID, album_id: ID) -> Self {
        AlbumUniqueIdentifier {
            artist_id: artist_id,
            album_id: album_id,
        }
    }
}

/// Identifier for a Track
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TrackUniqueIdentifier {
    pub artist_id: ID,
    pub album_id: ID,
    pub disc_no: u64,
    pub track_no: u64,
}

impl TrackUniqueIdentifier {
    pub fn new(artist_id: ID, album_id: ID, disc_no: u64, track_no: u64) -> Self {
        TrackUniqueIdentifier {
            artist_id: artist_id,
            album_id: album_id,
            disc_no: disc_no,
            track_no: track_no,
        }
    }

    pub fn from_track(track: &FullTrackMetadata) -> Self {
        TrackUniqueIdentifier {
            artist_id: track.album_artist_id.clone(),
            album_id: track.album_id.clone(),
            disc_no: track.disc,
            track_no: track.track,
        }
    }
}

/// Artist Info with Sorted Albums
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortedArtistAlbums<T> {
    pub artist_info: ArtistInfo,
    pub albums: Vec<SortedAlbumDiscs<T>>,
}

/// Album Info with Sorted Discs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortedAlbumDiscs<T> {
    pub album_info: AlbumInfo,
    pub path: path::PathBuf,
    pub discs: Vec<SortedDiscTracks<T>>,
}

/// Sorted Disc Tracks
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortedDiscTracks<T> {
    pub tracks: Vec<T>,
}
