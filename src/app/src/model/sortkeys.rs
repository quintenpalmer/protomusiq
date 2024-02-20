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

impl TrackSortKey {
    pub fn default_order(&self) -> SortOrder {
        match self {
            TrackSortKey::ByName => SortOrder::Regular,
            TrackSortKey::ByPlayCount => SortOrder::Reversed,
            TrackSortKey::ByDuration => SortOrder::Reversed,
            TrackSortKey::ByPlayedAmount => SortOrder::Reversed,
            TrackSortKey::ByRandom => SortOrder::Regular,
        }
    }
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

impl ArtistSortKey {
    pub fn default_order(&self) -> SortOrder {
        match self {
            ArtistSortKey::ByName => SortOrder::Regular,
            ArtistSortKey::ByPlayCount => SortOrder::Regular,
            ArtistSortKey::ByAlbumCount => SortOrder::Reversed,
            ArtistSortKey::ByTrackCount => SortOrder::Reversed,
            ArtistSortKey::ByTrackDuration => SortOrder::Reversed,
            ArtistSortKey::ByPlayedDuration => SortOrder::Reversed,
            ArtistSortKey::Random => SortOrder::Regular,
        }
    }
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
