use std::{path, pin};

use iced::futures::task::{Context, Poll};
use iced::futures::Future;

use crate::shared;

use crate::model;

use super::nav::*;

pub fn user_nav_message(nav: NavMessage) -> Message {
    Message::Nav(nav)
}

#[derive(Debug, Clone)]
pub enum Message {
    Action(Action),
    Nav(NavMessage),
    NavRelative(NavRelMsg),
    HistoryNav(HistoryDirection),
    PlaybackRequest(shared::PlaybackRequest),
    ErrorResponse(Result<(), String>),
    BackendCallback(shared::BackendToGUIMessage),
    ExternalSpawn(ExternalSpawn),
}

#[derive(Debug, Clone)]
pub enum HistoryDirection {
    Backwards,
    Forwards,
}

#[derive(Debug, Clone)]
pub enum Action {
    LoadEverything,

    ToggleFullscreen,

    CreateNewPlaylist(String),
    MakePlaylistDefault(u32),
    AddTracksToPlaylist(u32, Vec<musiqlibrary::TrackUniqueIdentifier>),
    DeletePlaylist(u32),
    RemoveTrackFromPlaylist(u32, musiqlibrary::TrackUniqueIdentifier),
    MoveTrackInPlaylist(u32, model::Direction, musiqlibrary::TrackUniqueIdentifier),

    ToggleShuffleOnAdd,

    SetVolume(VolumeRequest),

    UpdateText(String),
    PerformSearch(String, model::SearchDomain),
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
pub enum ExternalSpawn {
    Mpv(path::PathBuf),
    MGBA(path::PathBuf),
    ZSNES(path::PathBuf),
    Mupen64(path::PathBuf),
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
