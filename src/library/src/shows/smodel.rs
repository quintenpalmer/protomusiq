use std::collections::BTreeMap;
use std::{path, time};

use serde::{Deserialize, Serialize};

/// Possible Errors from Show Searching/Decoding
#[derive(Debug)]
pub enum Error {
    NonTextTitle,
    NonMP4File,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct ShowEpisodeKey {
    pub show: ShowKey,
    pub season_number: u32,
    pub episode_sort: u32,
}

#[derive(Debug, Clone)]
pub struct ShowMetadata {
    pub full_path: path::PathBuf,
    pub rel_path: path::PathBuf,
    pub last_modified: time::SystemTime,
    pub duration: time::Duration,

    pub show: String,                       // .tv_show_name
    pub album: String,                      // .album
    pub season_number: u32,                 // .tv_season
    pub grouping: Option<String>,           // .grouping
    pub episode_id: Option<String>,         // .tv_episode_name
    pub episode_sort: u32,                  // .tv_episode
    pub episode_sort_tiebreak: Option<u32>, // ??
    pub title: String,                      // .title
}

impl ShowMetadata {
    pub fn local_display_name(&self) -> String {
        match self.episode_id {
            Some(ref v) => v.clone(),
            None => format!("Episode: {}", self.episode_sort),
        }
    }

    pub fn get_key(&self) -> ShowEpisodeKey {
        ShowEpisodeKey {
            show: ShowKey {
                name: self.show.clone(),
            },
            season_number: self.season_number,
            episode_sort: self.episode_sort,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ShowKey {
    name: String,
}

impl ShowKey {
    pub fn raw_string(&self) -> &String {
        &self.name
    }
}

pub struct Shows {
    shows: BTreeMap<ShowKey, Show>,
}

impl Shows {
    pub fn from_vec(vec: &Vec<ShowMetadata>) -> Self {
        let mut shows = BTreeMap::new();

        for show_metadata in vec.iter() {
            let show = shows
                .entry(ShowKey {
                    name: show_metadata.show.clone(),
                })
                .or_insert(Show {
                    name: show_metadata.show.clone(),
                    album: show_metadata.album.clone(),
                    seasons: BTreeMap::new(),
                });

            show.add(&show_metadata);
        }

        Shows { shows }
    }

    pub fn get_shows(&self) -> &BTreeMap<ShowKey, Show> {
        &self.shows
    }

    pub fn get_show(&self, series_key: &ShowKey) -> Option<&Show> {
        self.shows.get(series_key)
    }
}

pub struct Show {
    name: String,
    #[allow(unused)]
    album: String,
    seasons: BTreeMap<u32, ShowSeason>,
}

impl Show {
    pub fn add(&mut self, metadata: &ShowMetadata) {
        let seasons = self
            .seasons
            .entry(metadata.season_number)
            .or_insert(ShowSeason {
                number: metadata.season_number,
                name: metadata.grouping.clone(),
                episodes: BTreeMap::new(),
            });

        seasons.add(&metadata);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_season(&self, season_id: &u32) -> Option<&ShowSeason> {
        self.seasons.get(season_id)
    }

    pub fn get_seasons(&self) -> &BTreeMap<u32, ShowSeason> {
        &self.seasons
    }

    pub fn get_first_season(&self) -> &ShowSeason {
        let (first_key, _) = self.seasons.first_key_value().unwrap();
        self.seasons.get(first_key).unwrap()
    }
}

pub struct ShowSeason {
    number: u32,
    name: Option<String>,
    episodes: BTreeMap<u32, ShowMetadata>,
}

impl ShowSeason {
    pub fn get_season_number(&self) -> u32 {
        self.number
    }

    pub fn get_episodes(&self) -> &BTreeMap<u32, ShowMetadata> {
        &self.episodes
    }

    pub fn get_episode(&self, episode_id: &u32) -> Option<&ShowMetadata> {
        self.episodes.get(episode_id)
    }

    pub fn get_first_episode(&self) -> &ShowMetadata {
        let (first_key, _) = self.episodes.first_key_value().unwrap();
        self.episodes.get(first_key).unwrap()
    }

    pub fn pretty_display(&self) -> String {
        match self.name {
            Some(ref name) => format!("{:02} : {}", self.number, name),
            None => format!("Season {:02}", self.number),
        }
    }

    fn add(&mut self, metadata: &ShowMetadata) {
        let maybe_conflict = self
            .episodes
            .insert(metadata.episode_sort, metadata.clone());
        match maybe_conflict {
            Some(conflict) => panic!(
                "found a conflict with {} {:?} {:?}",
                metadata.episode_sort, conflict, metadata
            ),
            None => (),
        }
    }
}
