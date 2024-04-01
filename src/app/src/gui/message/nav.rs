use musiqlibrary::video;

use crate::model;

#[derive(Debug, Clone)]
pub enum NavMessage {
    Home,
    Config,
    PlayQueueFocus,
    SearchPage(String, model::SearchDomain, bool),
    Playlist(PlaylistNavMessage),
    Music(MusicNavMessage),
    Movie(MovieNavMessage),
}

#[derive(Debug, Clone)]
pub enum MusicNavMessage {
    TrackList(usize, model::TrackSortKey, model::SortOrder),
    AlbumList(usize, model::AlbumSortKey, model::SortOrder),
    Artist(ArtistNavMessage),
}

#[derive(Debug, Clone)]
pub enum PlaylistNavMessage {
    PlaylistList(String),
    PlaylistView(u32),
}

#[derive(Debug, Clone)]
pub enum ArtistNavMessage {
    ArtistList(usize, model::ArtistSortKey, model::SortOrder),
    ArtistView(musiqlibrary::ID, ArtistViewType),
    AlbumView(musiqlibrary::ID, musiqlibrary::ID, ArtistAlbumView),
}

#[derive(Debug, Clone)]
pub enum ArtistViewType {
    ArtistAlbumsView,
    ArtistTrackView(model::ArtistTrackSortKey, model::SortOrder),
    ArtistFeaturedTrackView(model::ArtistFeaturedTrackSortKey, model::SortOrder),
    ArtistInfo,
    InPlaylist,
}

#[derive(Debug, Clone)]
pub enum ArtistAlbumView {
    ArtistAlbumTrackView(
        model::AlbumSize,
        Option<musiqlibrary::TrackUniqueIdentifier>,
        Option<model::AlbumSortPlacement>,
    ),
    InPlaylist,
}

#[derive(Debug, Clone)]
pub enum MovieNavMessage {
    MovieHome,
    MovieList(usize, model::MovieSortKey, model::SortOrder),
    MovieAttributes(Option<model::MovieAttribute>),
    MovieQuery(Option<model::MovieQueryParams>),
    MovieView(video::MovieMetadata, Option<model::MovieSize>),
}

#[derive(Debug, Clone)]
pub enum NavRelMsg {
    BreadcrumbSelection(usize),
    PagifiedMovement(PagifiedMovementMsg),
    SwitchSortBy(MoveDirectionMsg),
    ToggleSortOrder,
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
