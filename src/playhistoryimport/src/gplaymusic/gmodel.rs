use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use musiqlibrary;

pub static ALL_TRACKS_CSV: &'static str = "gplaymusic/input/all_tracks.csv";
pub static ALL_TRACKS_JSON: &'static str = "gplaymusic/input/all_tracks.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLineItem {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Album")]
    pub album: String,
    #[serde(rename = "Artist")]
    pub artist: String,
    #[serde(rename = "Duration (ms)")]
    pub duration_ms: String,
    #[serde(rename = "Rating")]
    pub rating: String,
    #[serde(rename = "Play Count")]
    pub play_count: String,
    #[serde(rename = "Removed")]
    pub removed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CleanedLineItem {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub play_count: u32,
}

impl CleanedLineItem {
    pub fn get_key(&self) -> GPlayMusicKey {
        GPlayMusicKey {
            title: self.title.to_lowercase(),
            album: self.album.to_lowercase(),
            artist: self.artist.to_lowercase(),
        }
    }

    pub fn get_key_with_mappings(
        &self,
        artist_mapping: &BTreeMap<String, musiqlibrary::ArtistInfo>,
        artist_album_mapping: &BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    ) -> GPlayMusicKey {
        let new_artist_match = match artist_mapping.get(&self.artist.to_lowercase()) {
            Some(artist_info) => artist_info.artist_name.to_lowercase(),
            None => self.artist.clone(),
        };

        let new_album_match = match artist_album_mapping
            .get(&(self.artist.to_lowercase(), self.album.to_lowercase()))
        {
            Some(artist_album_info) => artist_album_info.album.album_name.clone(),
            None => self.album.clone(),
        };

        GPlayMusicKey {
            title: self.title.to_lowercase(),
            album: new_album_match.to_lowercase(),
            artist: new_artist_match.to_lowercase(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GPlayMusicKey {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Album")]
    pub album: String,
    #[serde(rename = "Artist")]
    pub artist: String,
}

pub struct BestEffortMatchedInformation {
    pub all_zero_line_items: Vec<CleanedLineItem>,
    pub not_found: Vec<CleanedLineItem>,
    pub existing_library_with_zero_new_count:
        Vec<(musiqlibrary::FullTrackMetadata, (GPlayMusicKey, u32))>,
    pub manual_track_mapping: BTreeMap<GPlayMusicKey, (musiqlibrary::FullTrackMetadata, u32)>,
    pub manual_artist_mapping: BTreeMap<String, musiqlibrary::ArtistInfo>,
    pub manual_album_mapping: BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    pub manual_ignore_albums: BTreeSet<(String, String)>,
    pub ignore_album_info_with_play_counts: BTreeMap<(String, String), u32>,
    pub matched_tracks_json_ready: Vec<(musiqlibrary::FullTrackMetadata, (GPlayMusicKey, u32))>,
}

impl BestEffortMatchedInformation {
    pub fn sort_relevant(&mut self) {
        self.not_found.sort_by_key(|a| a.play_count);
        self.not_found.reverse();

        self.matched_tracks_json_ready.sort_by_key(|a| a.1 .1);
        self.matched_tracks_json_ready.reverse();
    }
}

pub fn compute_artist_play_count(albums: &Vec<(String, Vec<CleanedLineItem>)>) -> u32 {
    let mut total = 0;
    for (_album_name, tracks) in albums.iter() {
        for track in tracks.iter() {
            total += track.play_count;
        }
    }
    total
}

pub fn compute_album_play_count(tracks: &Vec<CleanedLineItem>) -> u32 {
    let mut total = 0;
    for track in tracks.iter() {
        total += track.play_count;
    }
    total
}
