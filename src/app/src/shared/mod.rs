use std::path;

use crate::model;

mod client;
mod state;

pub use client::{Callback, Client};
pub use state::{PlayQueueAction, PlayQueueEntry, PlayQueueInfo, PlayQueueTrack};

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

    SetVolume(f32),
    Close,
}

#[derive(Debug, Clone)]
pub enum GUIToBackendMessage {
    BackendPlayback(PlaybackRequest),
}

#[derive(Debug, Clone)]
pub enum BackendToGUIMessage {
    PlayQueueState(PlayQueueInfo),
}

#[derive(Debug, Clone)]
pub enum SinkMessage {
    PlayButton,
    PauseButton,
    LoadSong(path::PathBuf, f32),
    SetVolume(f32),
    Close,
}

#[derive(Debug, Clone)]
pub enum SinkCallbackMessage {
    Playing,
    Paused,
    SecondElapsed,
    SongEnded,
}

#[derive(Debug, Clone)]
pub enum MprisMessage {
    SetMetadata(String, String),
    SetPlaying,
    SetPaused,
    SetStopped,
    Close,
}

#[derive(Debug, Clone)]
pub enum MprisCallbackMessage {
    PlayPause,
    Play,
    Pause,
    Prev,
    Next,
}

#[derive(Debug, Clone)]
pub enum TrackerMessage {
    SongStarted(model::AugmentedTrack),
}
