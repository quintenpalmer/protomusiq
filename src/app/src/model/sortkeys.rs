#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TrackSortKey {
    ByName,
    ByPlayCount,
    ByDuration,
    ByPlayedAmount,
    Random,
}

impl TrackSortKey {
    pub fn default_order(&self) -> SortOrder {
        match self {
            TrackSortKey::ByName => SortOrder::Regular,
            TrackSortKey::ByPlayCount => SortOrder::Reversed,
            TrackSortKey::ByDuration => SortOrder::Reversed,
            TrackSortKey::ByPlayedAmount => SortOrder::Reversed,
            TrackSortKey::Random => SortOrder::Regular,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArtistTrackSortKey {
    ByName,
    ByParent,
    ByDuration,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    Random,
}

impl ArtistTrackSortKey {
    pub fn default_order(&self) -> SortOrder {
        match self {
            ArtistTrackSortKey::ByName => SortOrder::Regular,
            ArtistTrackSortKey::ByParent => SortOrder::Regular,
            ArtistTrackSortKey::ByDuration => SortOrder::Reversed,
            ArtistTrackSortKey::ByTotalPlayCount => SortOrder::Reversed,
            ArtistTrackSortKey::ByTotalPlayedDuration => SortOrder::Reversed,
            ArtistTrackSortKey::Random => SortOrder::Regular,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArtistFeaturedTrackSortKey {
    ByName,
    ByParent,
    ByDuration,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    Random,
}

impl ArtistFeaturedTrackSortKey {
    pub fn default_order(&self) -> SortOrder {
        match self {
            ArtistFeaturedTrackSortKey::ByName => SortOrder::Regular,
            ArtistFeaturedTrackSortKey::ByParent => SortOrder::Regular,
            ArtistFeaturedTrackSortKey::ByDuration => SortOrder::Reversed,
            ArtistFeaturedTrackSortKey::ByTotalPlayCount => SortOrder::Reversed,
            ArtistFeaturedTrackSortKey::ByTotalPlayedDuration => SortOrder::Reversed,
            ArtistFeaturedTrackSortKey::Random => SortOrder::Regular,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlbumSortKey {
    ByParent,
    ByName,
    ByLastMod,
    ByDuration,
    ByTotalPlayCount,
    ByTotalPlayedDuration,
    ByDate,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MovieSortKey {
    ByTitle,
    LastModified,
    ByDuration,
    ByRelease,
    Random,
}

impl MovieSortKey {
    pub fn default_order(&self) -> SortOrder {
        match self {
            MovieSortKey::ByTitle => SortOrder::Regular,
            MovieSortKey::LastModified => SortOrder::Reversed,
            MovieSortKey::ByDuration => SortOrder::Reversed,
            MovieSortKey::ByRelease => SortOrder::Reversed,
            MovieSortKey::Random => SortOrder::Regular,
        }
    }
}
