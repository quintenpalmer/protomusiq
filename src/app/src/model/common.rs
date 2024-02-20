use std::collections::BTreeMap;
use std::fs;
use std::path;

use serde::{Deserialize, Serialize};

use musiqlibrary::video;

use super::augmented;
use super::constants;

#[derive(Deserialize, Debug, Clone)]
pub enum LoadMode {
    NoCache,
    Latest,
    Json,
    Sqlite,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SortOrder {
    Regular,
    Reversed,
}

impl SortOrder {
    pub fn display_text(&self) -> String {
        match self {
            SortOrder::Regular => "Regular",
            SortOrder::Reversed => "Reversed",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TrackSortKey {
    ByName,
    ByPlayCount,
    ByDuration,
    ByPlayedAmount,
    ByRandom,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArtistTrackSortKey {
    ByName,
    ByParent,
    ByDuration,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    Random,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArtistFeaturedTrackSortKey {
    ByName,
    ByParent,
    ByDuration,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    Random,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArtistSortKey {
    ByName,
    ByPlayCount,
    ByAlbumCount,
    ByTrackCount,
    ByTrackDuration,
    ByPlayedDuration,
    Random,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

impl AlbumSortKey {
    pub fn prev(&self) -> Self {
        match self {
            AlbumSortKey::ByParent => AlbumSortKey::ByParent,
            AlbumSortKey::ByName => AlbumSortKey::ByParent,
            AlbumSortKey::ByLastMod => AlbumSortKey::ByName,
            AlbumSortKey::ByDuration => AlbumSortKey::ByLastMod,
            AlbumSortKey::ByTotalPlayCount => AlbumSortKey::ByDuration,
            AlbumSortKey::ByTotalPlayedDuration => AlbumSortKey::ByTotalPlayCount,
            AlbumSortKey::ByDate => AlbumSortKey::ByTotalPlayedDuration,
            AlbumSortKey::Random => AlbumSortKey::ByDate,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            AlbumSortKey::ByParent => AlbumSortKey::ByName,
            AlbumSortKey::ByName => AlbumSortKey::ByLastMod,
            AlbumSortKey::ByLastMod => AlbumSortKey::ByDuration,
            AlbumSortKey::ByDuration => AlbumSortKey::ByTotalPlayCount,
            AlbumSortKey::ByTotalPlayCount => AlbumSortKey::ByTotalPlayedDuration,
            AlbumSortKey::ByTotalPlayedDuration => AlbumSortKey::ByDate,
            AlbumSortKey::ByDate => AlbumSortKey::Random,
            AlbumSortKey::Random => AlbumSortKey::Random,
        }
    }

    pub fn default_order(&self) -> SortOrder {
        match self {
            AlbumSortKey::ByName => SortOrder::Regular,
            AlbumSortKey::ByParent => SortOrder::Regular,
            AlbumSortKey::ByDate => SortOrder::Reversed,
            AlbumSortKey::ByDuration => SortOrder::Reversed,
            AlbumSortKey::ByLastMod => SortOrder::Reversed,
            AlbumSortKey::ByTotalPlayCount => SortOrder::Reversed,
            AlbumSortKey::ByTotalPlayedDuration => SortOrder::Reversed,
            AlbumSortKey::Random => SortOrder::Regular,
        }
    }
}

impl AlbumSortKey {
    pub fn display_text(&self) -> String {
        match self {
            AlbumSortKey::ByName => "Name",
            AlbumSortKey::ByParent => "Artist",
            AlbumSortKey::ByDate => "Release Date",
            AlbumSortKey::ByDuration => "Length",
            AlbumSortKey::ByLastMod => "Added",
            AlbumSortKey::ByTotalPlayCount => "Play Count",
            AlbumSortKey::ByTotalPlayedDuration => "Played Duration",
            AlbumSortKey::Random => "Random",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MovieSortKey {
    ByTitle,
    LastModified,
    ByDuration,
    ByRelease,
    Random,
}

#[derive(Debug, Clone)]
pub struct AlbumSortPlacement {
    pub index: usize,
    pub sort_key: AlbumSortKey,
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SearchDomain {
    Music,
    Movies,
}

#[derive(Debug)]
pub struct PrehistoryRecord {
    pub source: String,
    pub key: musiqlibrary::TrackUniqueIdentifier,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub enum MovieSize {
    Micro,
    Small,
    Regular,
    SemiLarge,
    Large,
}

impl MovieSize {
    pub fn height(&self) -> u16 {
        match self {
            MovieSize::Micro => constants::DVD_MICRO_ICON_HEIGHT,
            MovieSize::Small => constants::DVD_SMALL_ICON_HEIGHT,
            MovieSize::Regular => constants::DVD_REGULAR_ICON_HEIGHT,
            MovieSize::SemiLarge => constants::DVD_SEMILARGE_ICON_HEIGHT,
            MovieSize::Large => constants::DVD_LARGE_ICON_HEIGHT,
        }
    }
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

pub struct ListAndReversed<T> {
    pub regular: Vec<T>,
    pub reversed: Vec<T>,
}

impl<T: Clone> ListAndReversed<T> {
    pub fn new(regular: Vec<T>) -> Self {
        let mut reversed = regular.clone();
        reversed.reverse();

        ListAndReversed { regular, reversed }
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
                self.large_album_covers
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
pub enum MovieQueryParams {
    Genre(String),
    Production(String),
    Producers(String),
    Director(String),
    Screenplay(String),
    CastMember(String),
}

#[derive(Debug, Clone)]
pub enum MovieAttribute {
    Genres,
    Production,
    Producers,
    Directors,
    Screenplay,
    CastMembers,
}

#[derive(Debug, Clone)]
pub enum AttributesList {
    Genre(Vec<String>),
    Production(Vec<String>),
    Producers(Vec<String>),
    Director(Vec<String>),
    Screenplay(Vec<String>),
    CastMember(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct SimpleSearchResults {
    pub artists: Vec<musiqlibrary::ArtistInfo>,
    pub albums: Vec<musiqlibrary::ArtistAlbumInfo>,
    pub tracks: Vec<augmented::AugmentedTrack>,
    pub track_artists: Vec<augmented::AugmentedTrack>,
}

#[derive(Debug, Clone)]
pub struct MovieSearchResults {
    pub titles: Vec<video::MovieMetadata>,
}
