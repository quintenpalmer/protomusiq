use std::collections::BTreeMap;
use std::{path, time};

/// Possible Errors from Show Searching/Decoding
#[derive(Debug)]
pub enum Error {
    NonTextTitle,
    NonMP4File,
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

pub struct Shows {
    pub shows: BTreeMap<String, Show>,
}

impl Shows {
    pub fn from_vec(vec: &Vec<ShowMetadata>) -> Self {
        let mut shows = BTreeMap::new();

        for show_metadata in vec.iter() {
            let show = shows.entry(show_metadata.show.clone()).or_insert(Show {
                name: show_metadata.show.clone(),
                album: show_metadata.album.clone(),
                seasons: BTreeMap::new(),
            });

            show.add(&show_metadata);
        }

        Shows { shows }
    }
}

pub struct Show {
    pub name: String,
    pub album: String,
    pub seasons: BTreeMap<u32, ShowSeason>,
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
}

pub struct ShowSeason {
    pub number: u32,
    pub name: Option<String>,
    pub episodes: BTreeMap<u32, ShowMetadata>,
}

impl ShowSeason {
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
