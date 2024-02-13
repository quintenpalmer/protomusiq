use musiqlibrary::{self, video};

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
    PlaylistList(PlaylistListState),
    PlaylistView(PlaylistViewState),
    TrackList(TrackListState),
    AlbumList(AlbumListState),
    ArtistList(ArtistListState),
    ArtistAlbumsView(ArtistViewState),
    ArtistTrackView(ArtistTrackViewState),
    ArtistFeaturedTrackView(ArtistFeaturedTrackViewState),
    ArtistAlbumView(ArtistAlbumViewState),
    MovieList(MovieListState),
    MovieView(MovieViewState),
    MovieQuery(MovieQueryState),
    MovieAttributes(MovieAttributeState),
}

impl Page {
    pub fn super_simple_debug_string(&self) -> String {
        match self {
            Page::Home(_) => "Home",
            Page::Config(_) => "Config",
            Page::Search(_) => "Search",
            Page::PlayQueue(_) => "PlayQueue",
            Page::PlaylistList(_) => "PlaylistList",
            Page::PlaylistView(_) => "PlaylistView",
            Page::TrackList(_) => "TrackList",
            Page::AlbumList(_) => "AlbumList",
            Page::ArtistList(_) => "ArtistList",
            Page::ArtistAlbumsView(_) => "ArtistAlbumsView",
            Page::ArtistTrackView(_) => "ArtistTrackViewState",
            Page::ArtistFeaturedTrackView(_) => "ArtistFeaturedTrackView",
            Page::ArtistAlbumView(_) => "ArtistAlbumView",
            Page::MovieList(_) => "MovieList",
            Page::MovieAttributes(_) => "MovieAttributes",
            Page::MovieQuery(_) => "MovieQuery",
            Page::MovieView(_) => "MovieView",
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
pub struct ArtistAlbumViewState {
    pub album_size: model::AlbumSize,
    pub artist_id: musiqlibrary::ID,
    pub album_id: musiqlibrary::ID,
    pub maybe_selected_track: Option<musiqlibrary::TrackUniqueIdentifier>,
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
}

#[derive(Debug)]
pub struct MovieQueryState {
    pub query: model::MovieQueryParams,
    pub matched_keys: Vec<model::MovieRelPath>,
}

#[derive(Debug)]
pub struct MovieAttributeState {
    pub attribute_results: model::AttributesList,
}
