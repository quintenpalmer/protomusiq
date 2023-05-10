use std::env;
use std::path;
use std::time;

use serde::{Deserialize, Serialize};

use musiqlibrary;

use super::constants;
use super::library;

#[derive(Deserialize, Debug, Clone)]
pub enum LoadMode {
    NoCache,
    Latest,
    JSON,
    Sqlite,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Regular,
    Reversed,
}

#[derive(Debug, Clone)]
pub enum TrackSortKey {
    ByName,
    ByPlayCount,
    ByDuration,
    ByPlayedAmount,
    ByRandom,
}

#[derive(Debug, Clone)]
pub enum ArtistTrackSortKey {
    ByName,
    ByParent,
    ByDuration,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    Random,
}

#[derive(Debug, Clone)]
pub enum ArtistSortKey {
    ByName,
    ByPlayCount,
    ByAlbumCount,
    ByTrackCount,
    ByTrackDuration,
    ByPlayedDuration,
    Random,
}

#[derive(Debug, Clone)]
pub enum AlbumSortKey {
    ByName,
    ByParent,
    ByDate,
    ByDuration,
    ByLastMod,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    Random,
}

#[derive(Debug, Clone)]
pub enum AlbumSize {
    Micro,
    Mini,
    Small,
    Regular,
    Large,
}

impl AlbumSize {
    pub fn width(&self) -> u16 {
        match self {
            AlbumSize::Micro => constants::MICRO_ICON_WIDTH,
            AlbumSize::Mini => constants::MINI_ICON_WIDTH,
            AlbumSize::Small => constants::SMALL_ICON_WIDTH,
            AlbumSize::Regular => constants::REGULAR_ICON_WIDTH,
            AlbumSize::Large => constants::LARGE_ICON_WIDTH,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            AlbumSize::Micro => constants::MICRO_ICON_HEIGHT,
            AlbumSize::Mini => constants::MINI_ICON_HEIGHT,
            AlbumSize::Small => constants::SMALL_ICON_HEIGHT,
            AlbumSize::Regular => constants::REGULAR_ICON_HEIGHT,
            AlbumSize::Large => constants::LARGE_ICON_HEIGHT,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum AlbumSizeWithOrig {
    Micro,
    Mini,
    Small,
    Regular,
    Large,
    Original,
}

#[derive(Debug)]
pub struct Pair<F, S> {
    pub first: F,
    pub second: S,
}

impl<F> Pair<F, ()> {
    pub fn new_empty(f: F) -> Self {
        Pair {
            first: f,
            second: (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleSearchResults {
    pub artists: Vec<musiqlibrary::ArtistInfo>,
    pub albums: Vec<musiqlibrary::ArtistAlbumInfo>,
    pub tracks: Vec<library::AugmentedTrack>,
    pub track_artists: Vec<library::AugmentedTrack>,
}

#[derive(Debug)]
pub struct SearchResults<T> {
    pub artists: Vec<Pair<musiqlibrary::ArtistInfo, T>>,
    pub albums: Vec<Pair<musiqlibrary::ArtistAlbumInfo, T>>,
    pub tracks: Vec<Pair<library::AugmentedTrack, T>>,
    pub track_artists: Vec<Pair<library::AugmentedTrack, T>>,
}

pub fn get_default_config_path() -> path::PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let config_path = path::Path::new(&home_dir)
        .join(".config")
        .join("musiqapp")
        .join("config.json");

    return config_path;
}

pub fn get_default_data_path() -> path::PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let data_path = path::Path::new(&home_dir)
        .join(".local")
        .join("share")
        .join("musiq")
        .join("v1");

    return data_path;
}

pub fn tracks_after_including(
    tracks: &Vec<library::AugmentedTrack>,
    current_track: &library::AugmentedTrack,
) -> Vec<library::AugmentedTrack> {
    let mut ret = Vec::new();
    let mut found = false;
    for iter_track in tracks.iter() {
        if !found {
            if iter_track == current_track {
                found = true;
            }
        }
        if found {
            ret.push(iter_track.clone());
        }
    }

    ret
}

pub fn compute_track_list_duration(tracks: &Vec<musiqlibrary::FullTrackMetadata>) -> time::Duration {
    let mut duration = time::Duration::new(0, 0);
    for track in tracks.iter() {
        duration += track.duration;
    }
    duration
}
