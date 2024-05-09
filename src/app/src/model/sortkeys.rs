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

    pub fn toggle(&self) -> Self {
        match self {
            SortOrder::Regular => SortOrder::Reversed,
            SortOrder::Reversed => SortOrder::Regular,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TrackSortKey {
    ByName,
    ByPlayCount,
    ByPlayedAmount,
    ByDuration,
    Random,
}

impl TrackSortKey {
    pub fn prev(&self) -> Self {
        match self {
            TrackSortKey::ByName => TrackSortKey::ByName,
            TrackSortKey::ByPlayCount => TrackSortKey::ByName,
            TrackSortKey::ByPlayedAmount => TrackSortKey::ByPlayCount,
            TrackSortKey::ByDuration => TrackSortKey::ByPlayedAmount,
            TrackSortKey::Random => TrackSortKey::ByDuration,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            TrackSortKey::ByName => TrackSortKey::ByPlayCount,
            TrackSortKey::ByPlayCount => TrackSortKey::ByPlayedAmount,
            TrackSortKey::ByPlayedAmount => TrackSortKey::ByDuration,
            TrackSortKey::ByDuration => TrackSortKey::Random,
            TrackSortKey::Random => TrackSortKey::Random,
        }
    }

    pub fn default_order(&self) -> SortOrder {
        match self {
            TrackSortKey::ByName => SortOrder::Regular,
            TrackSortKey::ByPlayCount => SortOrder::Reversed,
            TrackSortKey::ByPlayedAmount => SortOrder::Reversed,
            TrackSortKey::ByDuration => SortOrder::Reversed,
            TrackSortKey::Random => SortOrder::Regular,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArtistTrackSortKey {
    ByParent,
    ByName,
    ByTotalPlayCount,
    ByDuration,
    ByTotalPlayedDuration,
    Random,
}

impl ArtistTrackSortKey {
    pub fn prev(&self) -> Self {
        match self {
            ArtistTrackSortKey::ByParent => ArtistTrackSortKey::ByParent,
            ArtistTrackSortKey::ByName => ArtistTrackSortKey::ByParent,
            ArtistTrackSortKey::ByTotalPlayCount => ArtistTrackSortKey::ByName,
            ArtistTrackSortKey::ByDuration => ArtistTrackSortKey::ByTotalPlayCount,
            ArtistTrackSortKey::ByTotalPlayedDuration => ArtistTrackSortKey::ByDuration,
            ArtistTrackSortKey::Random => ArtistTrackSortKey::ByTotalPlayedDuration,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ArtistTrackSortKey::ByParent => ArtistTrackSortKey::ByName,
            ArtistTrackSortKey::ByName => ArtistTrackSortKey::ByTotalPlayCount,
            ArtistTrackSortKey::ByTotalPlayCount => ArtistTrackSortKey::ByDuration,
            ArtistTrackSortKey::ByDuration => ArtistTrackSortKey::ByTotalPlayedDuration,
            ArtistTrackSortKey::ByTotalPlayedDuration => ArtistTrackSortKey::Random,
            ArtistTrackSortKey::Random => ArtistTrackSortKey::Random,
        }
    }

    pub fn default_order(&self) -> SortOrder {
        match self {
            ArtistTrackSortKey::ByParent => SortOrder::Regular,
            ArtistTrackSortKey::ByName => SortOrder::Regular,
            ArtistTrackSortKey::ByTotalPlayCount => SortOrder::Reversed,
            ArtistTrackSortKey::ByDuration => SortOrder::Reversed,
            ArtistTrackSortKey::ByTotalPlayedDuration => SortOrder::Reversed,
            ArtistTrackSortKey::Random => SortOrder::Regular,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArtistFeaturedTrackSortKey {
    ByParent,
    ByName,
    ByTotalPlayCount,
    ByDuration,
    ByTotalPlayedDuration,
    Random,
}

impl ArtistFeaturedTrackSortKey {
    pub fn prev(&self) -> Self {
        match self {
            ArtistFeaturedTrackSortKey::ByParent => ArtistFeaturedTrackSortKey::ByParent,
            ArtistFeaturedTrackSortKey::ByName => ArtistFeaturedTrackSortKey::ByParent,
            ArtistFeaturedTrackSortKey::ByTotalPlayCount => ArtistFeaturedTrackSortKey::ByName,
            ArtistFeaturedTrackSortKey::ByDuration => ArtistFeaturedTrackSortKey::ByTotalPlayCount,
            ArtistFeaturedTrackSortKey::ByTotalPlayedDuration => {
                ArtistFeaturedTrackSortKey::ByDuration
            }
            ArtistFeaturedTrackSortKey::Random => ArtistFeaturedTrackSortKey::ByTotalPlayedDuration,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ArtistFeaturedTrackSortKey::ByParent => ArtistFeaturedTrackSortKey::ByName,
            ArtistFeaturedTrackSortKey::ByName => ArtistFeaturedTrackSortKey::ByTotalPlayCount,
            ArtistFeaturedTrackSortKey::ByTotalPlayCount => ArtistFeaturedTrackSortKey::ByDuration,
            ArtistFeaturedTrackSortKey::ByDuration => {
                ArtistFeaturedTrackSortKey::ByTotalPlayedDuration
            }
            ArtistFeaturedTrackSortKey::ByTotalPlayedDuration => ArtistFeaturedTrackSortKey::Random,
            ArtistFeaturedTrackSortKey::Random => ArtistFeaturedTrackSortKey::Random,
        }
    }

    pub fn default_order(&self) -> SortOrder {
        match self {
            ArtistFeaturedTrackSortKey::ByParent => SortOrder::Regular,
            ArtistFeaturedTrackSortKey::ByName => SortOrder::Regular,
            ArtistFeaturedTrackSortKey::ByTotalPlayCount => SortOrder::Reversed,
            ArtistFeaturedTrackSortKey::ByDuration => SortOrder::Reversed,
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
    pub fn prev(&self) -> Self {
        match self {
            ArtistSortKey::ByName => ArtistSortKey::ByName,
            ArtistSortKey::ByPlayCount => ArtistSortKey::ByName,
            ArtistSortKey::ByAlbumCount => ArtistSortKey::ByPlayCount,
            ArtistSortKey::ByTrackCount => ArtistSortKey::ByAlbumCount,
            ArtistSortKey::ByTrackDuration => ArtistSortKey::ByTrackCount,
            ArtistSortKey::ByPlayedDuration => ArtistSortKey::ByTrackDuration,
            ArtistSortKey::Random => ArtistSortKey::ByPlayedDuration,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ArtistSortKey::ByName => ArtistSortKey::ByPlayCount,
            ArtistSortKey::ByPlayCount => ArtistSortKey::ByAlbumCount,
            ArtistSortKey::ByAlbumCount => ArtistSortKey::ByTrackCount,
            ArtistSortKey::ByTrackCount => ArtistSortKey::ByTrackDuration,
            ArtistSortKey::ByTrackDuration => ArtistSortKey::ByPlayedDuration,
            ArtistSortKey::ByPlayedDuration => ArtistSortKey::Random,
            ArtistSortKey::Random => ArtistSortKey::Random,
        }
    }

    pub fn default_order(&self) -> SortOrder {
        match self {
            ArtistSortKey::ByName => SortOrder::Regular,
            ArtistSortKey::ByPlayCount => SortOrder::Reversed,
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

    pub fn preferred_home() -> Self {
        AlbumSortKey::ByLastMod
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
    pub fn prev(&self) -> Self {
        match self {
            MovieSortKey::ByTitle => MovieSortKey::ByTitle,
            MovieSortKey::LastModified => MovieSortKey::ByTitle,
            MovieSortKey::ByDuration => MovieSortKey::LastModified,
            MovieSortKey::ByRelease => MovieSortKey::ByDuration,
            MovieSortKey::Random => MovieSortKey::ByRelease,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            MovieSortKey::ByTitle => MovieSortKey::LastModified,
            MovieSortKey::LastModified => MovieSortKey::ByDuration,
            MovieSortKey::ByDuration => MovieSortKey::ByRelease,
            MovieSortKey::ByRelease => MovieSortKey::Random,
            MovieSortKey::Random => MovieSortKey::Random,
        }
    }

    pub fn default_order(&self) -> SortOrder {
        match self {
            MovieSortKey::ByTitle => SortOrder::Regular,
            MovieSortKey::LastModified => SortOrder::Reversed,
            MovieSortKey::ByDuration => SortOrder::Reversed,
            MovieSortKey::ByRelease => SortOrder::Reversed,
            MovieSortKey::Random => SortOrder::Regular,
        }
    }

    pub fn display_text(&self) -> String {
        match self {
            MovieSortKey::ByTitle => "Title",
            MovieSortKey::LastModified => "Modified",
            MovieSortKey::ByDuration => "Length",
            MovieSortKey::ByRelease => "Release",
            MovieSortKey::Random => "Random",
        }
        .to_string()
    }

    pub fn preferred_home() -> Self {
        MovieSortKey::LastModified
    }
}
