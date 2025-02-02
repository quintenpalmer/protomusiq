use std::collections::BTreeSet;

use musiqlibrary::video;

use crate::model;

use iced::widget::text_input;
use once_cell::sync;

pub static TEXT_INPUT_ID: sync::Lazy<text_input::Id> = sync::Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum Page {
    Home(HomeState),
    Config(ConfigState),
    PlayQueue(PlayQueueState),
    Search(SearchPageState),
    MusicHome,
    PlaylistList(PlaylistListState),
    PlaylistView(PlaylistViewState),
    GenreHome,
    TrackList(TrackListState),
    AlbumList(AlbumListState),
    ArtistList(ArtistListState),
    ArtistAlbumsView(ArtistViewState),
    ArtistInfoView(ArtistInfoState),
    ArtistTrackView(ArtistTrackViewState),
    ArtistFeaturedTrackView(ArtistFeaturedTrackViewState),
    ArtistFeaturedInPlaylist(ArtistFeaturedInPlaylistState),
    ArtistAlbumView(ArtistAlbumViewState),
    ArtistAlbumFeaturedInPlaylist(ArtistAlbumFeaturedInPlaylistState),
    MovieHome,
    MovieList(MovieListState),
    MovieView(MovieViewState),
    MovieQuery(MovieQueryState),
    MovieAttributes(MovieAttributeState),
    MovieSeriesList,
    GameHome,
    GBList,
    GBCList,
    GBAList,
    SNESList,
    N64List,
    NDSList,
    GameCubeList,
    WiiList,
    ShowHome,
    ShowList,
    ShowSeriesView(musiqlibrary::shows::ShowKey),
    ShowSeasonView(musiqlibrary::shows::ShowKey, u32),
    ShowContinueWatching,
}

impl Page {
    pub fn super_simple_debug_string(&self) -> String {
        match self {
            Page::Home(_) => "Home",
            Page::Config(_) => "Config",
            Page::Search(_) => "Search",
            Page::MusicHome => "MusicHome",
            Page::PlayQueue(_) => "PlayQueue",
            Page::PlaylistList(_) => "PlaylistList",
            Page::PlaylistView(_) => "PlaylistView",
            Page::GenreHome => "GenreHome",
            Page::TrackList(_) => "TrackList",
            Page::AlbumList(_) => "AlbumList",
            Page::ArtistList(_) => "ArtistList",
            Page::ArtistAlbumsView(_) => "ArtistAlbumsView",
            Page::ArtistTrackView(_) => "ArtistTrackViewState",
            Page::ArtistInfoView(_) => "ArtistInfoView",
            Page::ArtistFeaturedTrackView(_) => "ArtistFeaturedTrackView",
            Page::ArtistFeaturedInPlaylist(_) => "ArtistFeaturedInPlaylist",
            Page::ArtistAlbumView(_) => "ArtistAlbumView",
            Page::ArtistAlbumFeaturedInPlaylist(_) => "ArtistAlbumFeaturedInPlaylist",
            Page::MovieHome => "MovieHome",
            Page::MovieList(_) => "MovieList",
            Page::MovieAttributes(_) => "MovieAttributes",
            Page::MovieQuery(_) => "MovieQuery",
            Page::MovieView(_) => "MovieView",
            Page::MovieSeriesList => "MovieSeriesList",
            Page::GameHome => "GameHome",
            Page::GBList => "GBList",
            Page::GBCList => "GBCList",
            Page::GBAList => "GBAList",
            Page::SNESList => "SNESList",
            Page::N64List => "N64List",
            Page::NDSList => "NDSList",
            Page::GameCubeList => "GameCubeList",
            Page::WiiList => "WiiList",
            Page::ShowHome => "ShowHome",
            Page::ShowList => "ShowList",
            Page::ShowSeriesView(_) => "ShowSeriesView",
            Page::ShowSeasonView(_, _) => "ShowSeasonView",
            Page::ShowContinueWatching => "ShowContinueWatching",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct HomeState {}

#[derive(Debug)]
pub struct ConfigState {}

#[derive(Debug)]
pub struct SearchPageState {
    pub query: String,

    pub domain_results: SearchDomainResults,
}

#[derive(Debug)]
pub enum SearchDomainResults {
    Music(Option<model::SimpleSearchResults>),
    Movies(Option<model::MovieSearchResults>),
}

#[derive(Debug)]
pub struct PlayQueueState {}

#[derive(Debug)]
pub struct PlaylistListState {
    pub new_playlist_name: String,
}

#[derive(Debug)]
pub struct PlaylistViewState {
    pub playlist_id: u32,
}

#[derive(Debug)]
pub struct TrackListState {
    pub sort_key: model::TrackSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct AlbumListState {
    pub sort_key: model::AlbumSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct ArtistListState {
    pub sort_key: model::ArtistSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct ArtistViewState {
    pub artist_id: musiqlibrary::ID,
    pub albums: Vec<musiqlibrary::ID>,
}

#[derive(Debug)]
pub struct ArtistInfoState {
    pub artist_id: musiqlibrary::ID,
}

#[derive(Debug)]
pub struct ArtistTrackViewState {
    pub artist_id: musiqlibrary::ID,

    pub sort_key: model::ArtistTrackSortKey,
    pub sort_order: model::SortOrder,
}

#[derive(Debug)]
pub struct ArtistFeaturedTrackViewState {
    pub artist_id: musiqlibrary::ID,

    pub sort_key: model::ArtistFeaturedTrackSortKey,
    pub sort_order: model::SortOrder,
}

#[derive(Debug)]
pub struct ArtistFeaturedInPlaylistState {
    pub artist_id: musiqlibrary::ID,

    pub playlist_ids: BTreeSet<u32>,
}

#[derive(Debug)]
pub struct ArtistAlbumViewState {
    pub album_size: model::AlbumSize,
    pub artist_id: musiqlibrary::ID,
    pub album_id: musiqlibrary::ID,
    pub maybe_selected_track: Option<musiqlibrary::TrackUniqueIdentifier>,
    pub maybe_current_sort_order: Option<model::AlbumSortPlacement>,
}

#[derive(Debug)]
pub struct ArtistAlbumFeaturedInPlaylistState {
    pub artist_id: musiqlibrary::ID,
    pub album_id: musiqlibrary::ID,

    pub playlist_ids: BTreeSet<u32>,
}

#[derive(Debug)]
pub struct MovieListState {
    pub sort_key: model::MovieSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct MovieViewState {
    pub movie: video::MovieMetadata,
    pub movie_size: Option<model::MovieSize>,
    pub maybe_current_sort_order: Option<model::MovieSortPlacement>,
}

#[derive(Debug)]
pub struct MovieQueryState {
    pub query: Option<model::MovieQueryParams>,
    pub matched_keys: Option<Vec<model::MovieRelPath>>,
}

#[derive(Debug)]
pub struct MovieAttributeState {
    pub attribute_results: Option<model::AttributesList>,
}
