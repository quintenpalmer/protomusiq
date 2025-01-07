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
    Game(GameNavMessage),
    Shows(ShowNavMessage),
}

#[derive(Debug, Clone)]
pub enum MusicNavMessage {
    MusicHome,
    TrackList(usize, model::TrackSortKey, model::SortOrder),
    AlbumList(usize, model::AlbumSortKey, model::SortOrder),
    Artist(ArtistNavMessage),
    Genres(MusicGenreNavMessage),
}

#[derive(Debug, Clone)]
pub enum MusicGenreNavMessage {
    Home,
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
    MovieView(
        video::MovieMetadata,
        Option<model::MovieSize>,
        Option<model::MovieSortPlacement>,
    ),
    SeriesList,
}

#[derive(Debug, Clone)]
pub enum GameNavMessage {
    GameHome,
    GBList,
    GBCList,
    GBAList,
    SNESList,
    N64List,
    NDSList,
    GameCubeList,
    WiiList,
}

#[derive(Debug, Clone)]
pub enum ShowNavMessage {
    Home,
    ShowList,
    ShowSeries(musiqlibrary::shows::ShowKey),
    ShowSeason(musiqlibrary::shows::ShowKey, u32),
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
