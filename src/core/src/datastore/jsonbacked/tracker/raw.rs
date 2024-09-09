use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Default)]
pub struct RawTrackedPayload {
    pub tracks: Vec<(musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>)>,
}

impl RawTrackedPayload {
    pub fn to_btree_map(
        self,
    ) -> BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>> {
        self.tracks.into_iter().collect()
    }

    pub fn from_btree_map(
        tracks: &BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>>,
    ) -> Self {
        RawTrackedPayload {
            tracks: tracks
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        }
    }
}
