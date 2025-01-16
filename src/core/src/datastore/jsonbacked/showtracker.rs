use std::collections::BTreeMap;
use std::{cmp, fs, io, path};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::datastore::jsonbacked::common;

pub struct ShowTracker {
    show_tracker_json_file_path: path::PathBuf,
    tracked_show_views: BTreeMap<musiqlibrary::shows::ShowEpisodeKey, Vec<DateTime<Local>>>,
    cached_most_recently_viewed_shows: BTreeMap<String, musiqlibrary::shows::ShowEpisodeKey>,
}

impl ShowTracker {
    pub fn new<P: AsRef<path::Path>>(app_data_path: P) -> Self {
        let app_data_path = app_data_path.as_ref().to_path_buf();
        let (tracked_show_views_raw, show_tracker_json_file_path): (RawShowTrackedPayload, _) =
            common::bootstrap_raw_data(
                &app_data_path,
                vec!["data", "showtracker", "timestamps.json"],
            );

        let mut cached_most_recently_viewed_shows = BTreeMap::new();

        for (episode_key, _timestamp) in tracked_show_views_raw.views.iter() {
            let found_episode = cached_most_recently_viewed_shows
                .entry(episode_key.show.clone())
                .or_insert(episode_key.clone());

            *found_episode = cmp::max(found_episode.clone(), episode_key.clone());
        }

        let tracked_show_views = tracked_show_views_raw.to_btree_map();

        ShowTracker {
            show_tracker_json_file_path,
            tracked_show_views,
            cached_most_recently_viewed_shows,
        }
    }

    pub fn get_show_most_recently_viewed(
        &self,
        show_name: &String,
    ) -> Option<&musiqlibrary::shows::ShowEpisodeKey> {
        self.cached_most_recently_viewed_shows.get(show_name)
    }

    pub fn get_view_count_for_episode(&self, key: musiqlibrary::shows::ShowEpisodeKey) -> usize {
        match self.tracked_show_views.get(&key) {
            Some(views) => views.len(),
            None => 0,
        }
    }

    pub fn mark_episode_viewed_now(&mut self, key: musiqlibrary::shows::ShowEpisodeKey) {
        self.mark_episode_viewed_at(key, Local::now())
    }

    pub fn mark_episode_viewed_at(
        &mut self,
        key: musiqlibrary::shows::ShowEpisodeKey,
        view_time: DateTime<Local>,
    ) {
        self.tracked_show_views
            .entry(key)
            .or_insert(Vec::new())
            .push(view_time);

        let raw_tracker = RawShowTrackedPayload::from_btree_map(&self.tracked_show_views);

        serde_json::to_writer(
            io::BufWriter::new(fs::File::create(&self.show_tracker_json_file_path).unwrap()),
            &raw_tracker,
        )
        .unwrap();
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
        views: &BTreeMap<musiqlibrary::shows::ShowEpisodeKey, Vec<DateTime<Local>>>,
    ) -> Self {
        RawShowTrackedPayload {
            views: views.iter().map(|(x, y)| (x.clone(), y.clone())).collect(),
        }
    }
}
