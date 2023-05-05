use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use musiqlibrary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataPlayInfo {
    pub key: String,
    #[serde(rename = "playCount")]
    pub play_count: u32,
}

pub struct ResultingInformation {
    pub all_zero_line_items: Vec<UserDataPlayInfo>,
    pub uuid_line_items: Vec<UserDataPlayInfo>,
    pub existing_library_with_zero_new_count: Vec<(musiqlibrary::FullTrackMetadata, (String, u32))>,
    pub not_found: Vec<UserDataPlayInfo>,
    pub manual_mapping: BTreeMap<String, (musiqlibrary::FullTrackMetadata, u32)>,
    pub matched_tracks_json_ready: Vec<(musiqlibrary::FullTrackMetadata, (String, u32))>,
}

impl ResultingInformation {
    pub fn sort_relevant(&mut self) {
        self.uuid_line_items.sort_by_key(|a| a.play_count);
        self.uuid_line_items.reverse();

        self.not_found.sort_by_key(|a| a.play_count);
        self.not_found.reverse();

        self.matched_tracks_json_ready.sort_by_key(|a| a.1 .1);
        self.matched_tracks_json_ready.reverse();
    }
}
