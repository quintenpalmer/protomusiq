use std::path;

use crate::model;

mod client;

pub use client::{Callback, Client};

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

/// State for the Play Queue (and Current Track and Play History)
#[derive(Debug, Clone)]
pub struct PlayQueueInfo {
    pub current_second: u64,
    pub playing: bool,
    pub current_volume: f32,
    pub play_history: Vec<PlayQueueEntry>,
    pub current_playback: Option<PlayQueueEntry>,
    pub play_queue: Vec<PlayQueueEntry>,
}

impl PlayQueueInfo {
    pub fn new() -> Self {
        PlayQueueInfo {
            current_second: 0,
            playing: false,
            current_volume: 1.0,
            play_history: Vec::new(),
            current_playback: None,
            play_queue: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlayQueueEntry {
    Track(PlayQueueTrack),
    Action(PlayQueueAction),
}

#[derive(Debug, Clone)]
pub struct PlayQueueTrack {
    pub track: model::AugmentedTrack,
}

#[derive(Debug, Clone)]
pub enum PlayQueueAction {
    Pause,
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
