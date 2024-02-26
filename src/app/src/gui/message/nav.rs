use musiqlibrary::video;

use crate::model;

#[derive(Debug, Clone)]
pub enum NavMessage {
    Home,
    Config,
    PlayQueueFocus,
    SearchPage(String, model::SearchDomain, bool),
    PlaylistList(String),
    PlaylistView(u32),
    TrackList(usize, model::TrackSortKey, model::SortOrder),
    AlbumList(usize, model::AlbumSortKey, model::SortOrder),
    ArtistList(usize, model::ArtistSortKey, model::SortOrder),
    ArtistAlbumsView(musiqlibrary::ID),
    ArtistTrackView(
        musiqlibrary::ID,
        model::ArtistTrackSortKey,
        model::SortOrder,
    ),
    ArtistFeaturedTrackView(
        musiqlibrary::ID,
        model::ArtistFeaturedTrackSortKey,
        model::SortOrder,
    ),
    ArtistAlbumView(
        musiqlibrary::ID,
        musiqlibrary::ID,
        model::AlbumSize,
        Option<musiqlibrary::TrackUniqueIdentifier>,
        Option<model::AlbumSortPlacement>,
    ),
    MovieHome,
    MovieList(usize, model::MovieSortKey, model::SortOrder),
    MovieAttributes(Option<model::MovieAttribute>),
    MovieQuery(Option<model::MovieQueryParams>),
    MovieView(video::MovieMetadata, Option<model::MovieSize>),
}

#[derive(Debug, Clone)]
pub enum NavRelMsg {
    PagifiedMovement(PagifiedMovementMsg),
    SwitchSortBy(MoveDirectionMsg),
}

#[derive(Debug, Clone)]
pub enum PagifiedMovementMsg {
    First,
    Backwards,
    Forwards,
    Last,
}

#[derive(Debug, Clone)]
pub enum MoveDirectionMsg {
    Left,
    Right,
}
