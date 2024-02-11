use std::{path, pin};

use musiqlibrary::video;

use iced;

use iced::futures::task::{Context, Poll};
use iced::futures::Future;

use musiqlibrary;

use crate::shared;

use crate::model;

pub fn user_nav_message(nav: NavMessage) -> Message {
    Message::Nav(nav)
}

#[derive(Debug, Clone)]
pub enum Message {
    Action(Action),
    Nav(NavMessage),
    HistoryNav,
    PlaybackRequest(shared::PlaybackRequest),
    ErrorResponse(Result<(), String>),
    BackendCallback(shared::BackendToGUIMessage),
    ExternalSpawn(ExternalSpawn),
}

#[derive(Debug, Clone)]
pub enum Action {
    LoadEverything,

    CreateNewPlaylist(String),
    MakePlaylistDefault(u32),
    AddTracksToPlaylist(u32, Vec<musiqlibrary::TrackUniqueIdentifier>),
    DeletePlaylist(u32),
    RemoveTrackFromPlaylist(u32, musiqlibrary::TrackUniqueIdentifier),
    MoveTrackInPlaylist(u32, model::Direction, musiqlibrary::TrackUniqueIdentifier),

    ToggleShuffleOnAdd,

    SetVolume(VolumeRequest),

    UpdateText(String),
    PerformSearch(String, SearchDomain),
    TogglePlayQueueVisible,
    Notify(NotificationMessage),
    Close,
}

#[derive(Debug, Clone)]
pub enum NotificationMessage {
    OnScreen(NotificationAction),
    PopOnScreen,
    ClearOnScreen,
}

#[derive(Debug, Clone)]
pub enum NotificationAction {
    AddedToPlayQueue(String),
    AddedToPlaylist(String, String),
}

#[derive(Debug, Clone)]
pub enum VolumeRequest {
    Up(f32),
    Down(f32),
    Set(f32),
}

#[derive(Debug, Clone)]
pub enum NavMessage {
    Home,
    Config,
    PlayQueueFocus,
    SearchPage(String, SearchDomain, bool),
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
    ),
    MovieList(usize, model::MovieSortKey, model::SortOrder),
    MovieView(video::MovieMetadata, model::MovieSize),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SearchDomain {
    Music,
    Movies,
}

#[derive(Debug, Clone)]
pub enum ExternalSpawn {
    Mpv(path::PathBuf),
}

pub struct MessageFuture {
    pub inner: Message,
}

impl Future for MessageFuture {
    type Output = Message;
    fn poll(self: pin::Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.inner.clone())
    }
}
