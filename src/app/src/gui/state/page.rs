use musiqlibrary;

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
    ArtistView(ArtistViewState),
    ArtistTrackView(ArtistTrackViewState),
    ArtistAlbumView(ArtistAlbumViewState),
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
            Page::ArtistView(_) => "ArtistView",
            Page::ArtistTrackView(_) => "ArtistTrackViewState",
            Page::ArtistAlbumView(_) => "ArtistAlbumView",
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
    pub results: Option<model::SearchResults<()>>,

    pub query: String,
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
pub struct ArtistAlbumViewState {
    pub album_size: model::AlbumSize,
    pub artist_id: musiqlibrary::ID,
    pub album_id: musiqlibrary::ID,
    pub maybe_selected_track: Option<musiqlibrary::TrackUniqueIdentifier>,
}
