use std::collections::BTreeMap;
use std::fs;
use std::path;

use serde::{Deserialize, Serialize};

use musiqlibrary;

use super::augmented;
use super::constants;

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
pub enum ArtistFeaturedTrackSortKey {
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
pub enum MovieSortKey {
    ByTitle,
    Random,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub struct PrehistoryRecord {
    pub source: String,
    pub key: musiqlibrary::TrackUniqueIdentifier,
    pub count: u32,
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

pub struct ListAndReversed<T> {
    pub regular: Vec<T>,
    pub reversed: Vec<T>,
}

impl<T: Clone> ListAndReversed<T> {
    pub fn new(regular: Vec<T>) -> Self {
        let mut reversed = regular.clone();
        reversed.reverse();

        ListAndReversed {
            regular: regular,
            reversed: reversed,
        }
    }

    pub fn sort_ordered(&self, sort_order: &SortOrder) -> &Vec<T> {
        match sort_order {
            SortOrder::Regular => &self.regular,
            SortOrder::Reversed => &self.reversed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlbumArt {
    pub large_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, path::PathBuf>,
    pub album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub small_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub mini_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub micro_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
}

impl AlbumArt {
    pub fn get_album_cover(
        &self,
        album_size: AlbumSize,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> Vec<u8> {
        match album_size {
            AlbumSize::Large => fs::read(
                &self
                    .large_album_covers
                    .get(&musiqlibrary::AlbumUniqueIdentifier::new(
                        artist_id, album_id,
                    ))
                    .unwrap(),
            )
            .unwrap(),
            AlbumSize::Regular => self
                .album_covers
                .get(&musiqlibrary::AlbumUniqueIdentifier::new(
                    artist_id, album_id,
                ))
                .unwrap()
                .clone(),
            AlbumSize::Small => self
                .small_album_covers
                .get(&musiqlibrary::AlbumUniqueIdentifier::new(
                    artist_id, album_id,
                ))
                .unwrap()
                .clone(),
            AlbumSize::Mini => self
                .mini_album_covers
                .get(&musiqlibrary::AlbumUniqueIdentifier::new(
                    artist_id, album_id,
                ))
                .unwrap()
                .clone(),
            AlbumSize::Micro => self
                .micro_album_covers
                .get(&musiqlibrary::AlbumUniqueIdentifier::new(
                    artist_id, album_id,
                ))
                .unwrap()
                .clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleSearchResults {
    pub artists: Vec<musiqlibrary::ArtistInfo>,
    pub albums: Vec<musiqlibrary::ArtistAlbumInfo>,
    pub tracks: Vec<augmented::AugmentedTrack>,
    pub track_artists: Vec<augmented::AugmentedTrack>,
}

#[derive(Debug)]
pub struct SearchResults<T> {
    pub artists: Vec<Pair<musiqlibrary::ArtistInfo, T>>,
    pub albums: Vec<Pair<musiqlibrary::ArtistAlbumInfo, T>>,
    pub tracks: Vec<Pair<augmented::AugmentedTrack, T>>,
    pub track_artists: Vec<Pair<augmented::AugmentedTrack, T>>,
}
