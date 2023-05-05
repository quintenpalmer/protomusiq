use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLineItem {
    // format: "2013-10-18T16:49:31Z",
    pub ts: String,
    pub ms_played: u32,
    pub master_metadata_track_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    pub master_metadata_album_album_name: Option<String>,
    pub reason_start: String,
    pub reason_end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanedLineItem {
    pub ts: chrono::DateTime<chrono::Local>,
    pub ms_played: u32,
    pub master_metadata_track_name: String,
    pub master_metadata_album_artist_name: String,
    pub master_metadata_album_album_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLineItemSet {
    pub master_metadata_track_name: String,
    pub master_metadata_album_artist_name: String,
    pub master_metadata_album_album_name: String,
    pub plays: Vec<SinglePlay>,
}

impl StructuredLineItemSet {
    pub fn _new_empty(clean: &CleanedLineItem) -> Self {
        StructuredLineItemSet {
            master_metadata_track_name: clean.master_metadata_track_name.to_lowercase(),
            master_metadata_album_artist_name: clean
                .master_metadata_album_artist_name
                .to_lowercase(),
            master_metadata_album_album_name: clean.master_metadata_album_album_name.to_lowercase(),
            plays: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinglePlay {
    pub ts: chrono::DateTime<chrono::Local>,
    pub ms_played: u32,
}

impl SinglePlay {
    pub fn _new_from(clean: &CleanedLineItem) -> Self {
        SinglePlay {
            ts: clean.ts.clone(),
            ms_played: clean.ms_played.clone(),
        }
    }
}

impl CleanedLineItem {
    pub fn get_key(&self) -> SpotifyKey {
        SpotifyKey {
            master_metadata_track_name: self.master_metadata_track_name.to_lowercase(),
            master_metadata_album_artist_name: self
                .master_metadata_album_artist_name
                .to_lowercase(),
            master_metadata_album_album_name: self.master_metadata_album_album_name.to_lowercase(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpotifyKey {
    pub master_metadata_track_name: String,
    pub master_metadata_album_artist_name: String,
    pub master_metadata_album_album_name: String,
}

impl SpotifyKey {
    pub fn get_key_with_mappings(
        &self,
        artist_album_mapping: &BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    ) -> SpotifyKey {
        let (new_artist_match, new_album_match) = match artist_album_mapping.get(&(
            self.master_metadata_album_artist_name.to_lowercase(),
            self.master_metadata_album_album_name.to_lowercase(),
        )) {
            Some(artist_album_info) => (
                artist_album_info.artist.artist_name.clone(),
                artist_album_info.album.album_name.clone(),
            ),
            None => (
                self.master_metadata_album_artist_name.to_lowercase(),
                self.master_metadata_album_album_name.to_lowercase(),
            ),
        };

        SpotifyKey {
            master_metadata_track_name: self.master_metadata_track_name.to_lowercase(),
            master_metadata_album_album_name: new_album_match.to_lowercase(),
            master_metadata_album_artist_name: new_artist_match.to_lowercase(),
        }
    }
}

pub struct DebugInfo {
    pub play_info_lines_count: Vec<(SpotifyKey, usize)>,
    pub not_found: BTreeMap<SpotifyKey, Vec<CleanedLineItem>>,
    pub not_found_albums: BTreeMap<(String, String), usize>,
    pub keyed_library_items: BTreeMap<SpotifyKey, musiqlibrary::FullTrackMetadata>,
    pub manual_track_mapping:
        BTreeMap<SpotifyKey, (musiqlibrary::FullTrackMetadata, Vec<CleanedLineItem>)>,
    pub manual_album_mapping:
        BTreeMap<(std::string::String, std::string::String), musiqlibrary::ArtistAlbumInfo>,
    pub manual_ignore_albums: BTreeSet<(std::string::String, std::string::String)>,
    pub found_keys_in_library_matches:
        BTreeMap<SpotifyKey, (musiqlibrary::FullTrackMetadata, Vec<CleanedLineItem>)>,
}

pub fn compute_album_play_count(tracks: &Vec<CleanedLineItem>) -> usize {
    let mut total = 0;
    for _track in tracks.iter() {
        total += 1;
    }
    total
}
