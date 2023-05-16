use std::pin;

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
    PlaybackRequest(PlaybackRequest),
    ErrorResponse(Result<(), String>),
    SinkCallback(shared::SinkCallbackMessage),
    MprisCallback(shared::MprisCallbackMessage),
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum Action {
    LoadEverything,

    CreateNewPlaylist(String),
    MakePlaylistDefault(u64),
    AddTracksToPlaylist(u64, Vec<musiqlibrary::TrackUniqueIdentifier>),
    DeletePlaylist(u64),
    RemoveTrackFromPlaylist(u64, musiqlibrary::TrackUniqueIdentifier),
    MoveTrackInPlaylist(u64, Direction, musiqlibrary::TrackUniqueIdentifier),

    RemoveTrackFromPlayQueue(HistoryOrQueue, usize),

    ToggleShuffleOnAdd,

    SetVolume(VolumeRequest),

    UpdateText(String),
    PerformSearch(String),
    TogglePlayQueueVisible,
    Close,
}

#[derive(Debug, Clone)]
pub enum PlaybackRequest {
    Play,
    Pause,
    Prev,
    Next,

    InsertPause,

    LoadCurrentSong,
    PlaySongs(Vec<model::AugmentedTrack>),
    InsertSongs(Vec<model::AugmentedTrack>, bool),
    AppendSongs(Vec<model::AugmentedTrack>, bool),
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
    SearchPage(String, bool),
    PlaylistList(String),
    PlaylistView(u64),
    TrackList(usize, model::TrackSortKey, model::SortOrder),
    AlbumList(usize, model::AlbumSortKey, model::SortOrder),
    ArtistList(usize, model::ArtistSortKey, model::SortOrder),
    ArtistView(musiqlibrary::ID),
    ArtistTrackView(
        musiqlibrary::ID,
        model::ArtistTrackSortKey,
        model::SortOrder,
    ),
    ArtistAlbumView(musiqlibrary::ID, musiqlibrary::ID, model::AlbumSize, Option<musiqlibrary::TrackUniqueIdentifier>),
}

#[derive(Debug, Clone)]
pub enum HistoryOrQueue {
    History,
    Queue,
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
