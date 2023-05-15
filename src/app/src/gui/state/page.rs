use std::collections::BTreeMap;

use iced::{button, scrollable, text_input};

use musiqlibrary;

use crate::model;

#[derive(Debug)]
pub struct PlayQueueInteractionButtons {
    pub play_button: button::State,
    pub insert_button: button::State,
    pub append_button: button::State,
}

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
pub struct HomeState {
    pub artist_list_button: button::State,
    pub album_list_button: button::State,
    pub track_list_button: button::State,
    pub playlist_list_button: button::State,
    pub scroll: scrollable::State,
}

#[derive(Debug)]
pub struct ConfigState {
    pub refresh_library_button: button::State,
}

#[derive(Debug)]
pub struct SearchPageState {
    pub input_state: text_input::State,
    pub artist_scroll: scrollable::State,
    pub album_scroll: scrollable::State,
    pub track_scroll: scrollable::State,
    pub track_artist_scroll: scrollable::State,

    pub results: model::SearchResults<button::State>,

    pub query: String,
}

#[derive(Debug)]
pub struct PlayQueueState {}

#[derive(Debug)]
pub struct PlaylistListState {
    pub playlist_scroll: scrollable::State,
    pub playlist_list_breadcrumb: button::State,
    pub playlist_make_default_buttons: Vec<PlaylistListButtons>,
    pub new_playlist_name: String,
    pub new_playlist_text_input: text_input::State,
    pub new_playlist_button: button::State,
}

#[derive(Debug)]
pub struct PlaylistListButtons {
    pub link_to_playlist_button: button::State,
    pub delete_playlist_button: button::State,
    pub make_default_button: button::State,
}

#[derive(Debug)]
pub struct PlaylistViewState {
    pub playlist_play_queue_buttons: PlayQueueInteractionButtons,
    pub track_play_buttons: Vec<PlaylistTrackLineItemButtons>,
    pub playlist_list_breadcrumb: button::State,
    pub this_playlist_breadcrumb: button::State,
    pub track_scroll: scrollable::State,

    pub playlist_id: u64,
}

#[derive(Debug)]
pub struct PlaylistTrackLineItemButtons {
    pub play_button: button::State,
    pub link_button: button::State,
    pub remove_from_playlist_button: button::State,
    pub move_up_in_playlist_button: button::State,
    pub move_down_in_playlist_button: button::State,
    pub insert_button: button::State,
    pub append_button: button::State,
}

#[derive(Debug)]
pub struct TrackListState {
    pub album_list_breadcrumb: button::State,
    pub sort_order_regular_button: button::State,
    pub sort_order_reverse_button: button::State,
    pub sort_by_name_button: button::State,
    pub sort_by_play_count_button: button::State,
    pub sort_by_duration_button: button::State,
    pub sort_by_played_duration_button: button::State,
    pub sort_random_button: button::State,
    pub nav_first_button: button::State,
    pub nav_back_button: button::State,
    pub nav_forward_button: button::State,
    pub nav_last_button: button::State,
    pub album_buttons: Vec<button::State>,
    pub track_scroll: scrollable::State,

    pub sort_key: model::TrackSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct AlbumListState {
    pub album_list_breadcrumb: button::State,
    pub sort_order_regular_button: button::State,
    pub sort_order_reverse_button: button::State,
    pub sort_by_name_button: button::State,
    pub sort_by_date_button: button::State,
    pub sort_by_duration_button: button::State,
    pub sort_by_last_mod_button: button::State,
    pub sort_by_total_play_count_button: button::State,
    pub sort_by_total_played_duration_button: button::State,
    pub sort_random_button: button::State,
    pub sort_by_artist_button: button::State,
    pub nav_first_button: button::State,
    pub nav_back_button: button::State,
    pub nav_forward_button: button::State,
    pub nav_last_button: button::State,
    pub album_buttons: Vec<button::State>,
    pub album_scroll: scrollable::State,

    pub sort_key: model::AlbumSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct ArtistListState {
    pub artist_list_breadcrumb: button::State,
    pub sort_order_regular_button: button::State,
    pub sort_order_reverse_button: button::State,

    pub sort_by_name_button: button::State,
    pub sort_random_button: button::State,
    pub sort_by_play_count_button: button::State,
    pub sort_by_album_count_button: button::State,
    pub sort_by_track_count_button: button::State,
    pub sort_by_track_duration_button: button::State,
    pub sort_by_duration_played_button: button::State,

    pub nav_first_button: button::State,
    pub nav_back_button: button::State,
    pub nav_forward_button: button::State,
    pub nav_last_button: button::State,
    pub artist_buttons: Vec<button::State>,
    pub artist_scroll: scrollable::State,

    pub sort_key: model::ArtistSortKey,
    pub sort_order: model::SortOrder,
    pub page: usize,
}

#[derive(Debug)]
pub struct ArtistViewState {
    pub artist_list_breadcrumb: button::State,
    pub artist_view_breadcrumb: button::State,
    pub album_scroll: scrollable::State,

    pub album_view_button: button::State,
    pub track_view_button: button::State,

    pub artist_id: musiqlibrary::ID,
    pub album_buttons: BTreeMap<musiqlibrary::ID, button::State>,
}

#[derive(Debug)]
pub struct ArtistTrackViewState {
    pub artist_list_breadcrumb: button::State,
    pub artist_view_breadcrumb: button::State,

    pub sort_by_name_button: button::State,
    pub sort_by_album_button: button::State,
    pub sort_by_play_count_button: button::State,
    pub sort_by_duration_button: button::State,
    pub sort_by_played_duration_button: button::State,
    pub sort_random_button: button::State,

    pub album_view_button: button::State,
    pub track_view_button: button::State,

    pub sort_order_regular_button: button::State,
    pub sort_order_reverse_button: button::State,

    pub track_buttons: Vec<button::State>,
    pub track_scroll: scrollable::State,

    pub artist_id: musiqlibrary::ID,

    pub sort_key: model::ArtistTrackSortKey,
    pub sort_order: model::SortOrder,
}

#[derive(Debug)]
pub struct ArtistAlbumViewState {
    pub artist_list_breadcrumb: button::State,
    pub artist_view_breadcrumb: button::State,
    pub artist_album_view_breadcrumb: button::State,
    pub entire_track_list_buttons: TrackLineItemButtons,
    pub all_disc_buttons: Vec<TrackLineItemButtons>,
    pub toggle_image_size_button: button::State,
    pub track_play_buttons: Vec<Vec<TrackLineItemButtons>>,
    pub scroll: scrollable::State,

    pub album_size: model::AlbumSize,
    pub artist_id: musiqlibrary::ID,
    pub album_id: musiqlibrary::ID,
    pub maybe_selected_track: Option<musiqlibrary::TrackUniqueIdentifier>,
}

#[derive(Debug)]
pub struct TrackLineItemButtons {
    pub play_button: button::State,
    pub play_all_from_here_button: button::State,
    pub add_to_default_playlist_button: button::State,
    pub insert_button: button::State,
    pub append_button: button::State,
}
