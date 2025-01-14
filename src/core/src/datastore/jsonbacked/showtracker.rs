use std::collections::BTreeMap;
use std::path;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::datastore::jsonbacked::common;

pub struct ShowTracker {
    pub show_tracker_json_file_path: path::PathBuf,
    pub tracked_show_views: BTreeMap<musiqlibrary::shows::ShowEpisodeKey, Vec<DateTime<Local>>>,
}

impl ShowTracker {
    pub fn new<P: AsRef<path::Path>>(app_data_path: P) -> Self {
        let app_data_path = app_data_path.as_ref().to_path_buf();
        let (tracked_show_views, show_tracker_json_file_path): (RawShowTrackedPayload, _) =
            common::bootstrap_raw_data(
                &app_data_path,
                vec!["data", "showtracker", "timestamps.json"],
            );

        ShowTracker {
            show_tracker_json_file_path,
            tracked_show_views: tracked_show_views.to_btree_map(),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct RawShowTrackedPayload {
    pub views: Vec<(musiqlibrary::shows::ShowEpisodeKey, Vec<DateTime<Local>>)>,
}

impl RawShowTrackedPayload {
    pub fn to_btree_map(
        self,
    ) -> BTreeMap<musiqlibrary::shows::ShowEpisodeKey, Vec<DateTime<Local>>> {
        self.views.into_iter().collect()
    }

    pub fn from_btree_map(
        views: BTreeMap<musiqlibrary::shows::ShowEpisodeKey, Vec<DateTime<Local>>>,
    ) -> Self {
        RawShowTrackedPayload {
            views: views.into_iter().collect(),
        }
    }
}
