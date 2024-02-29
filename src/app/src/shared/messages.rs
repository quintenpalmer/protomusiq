use std::path;

use crate::model;

use super::state;

#[derive(Debug, Clone)]
pub enum GUIToBackendMessage {
    BackendPlayback(PlaybackRequest),
}

#[derive(Debug, Clone)]
pub enum PlaybackRequest {
    PlayPauseToggle,
    Play,
    Pause,
    Prev,
    Next,

    InsertPause,

    LoadCurrentSong,
    PlaySongs(Vec<model::AugmentedTrack>),
    InsertSongs(Vec<model::AugmentedTrack>, bool),
    AppendSongs(Vec<model::AugmentedTrack>, bool),

    RemoveTrackFromPlayQueue(HistoryOrQueue, usize),

    SetVolume(f32),
    Close,
}

#[derive(Debug, Clone)]
pub enum HistoryOrQueue {
    History,
    Queue,
}

#[derive(Debug, Clone)]
pub enum BackendToGUIMessage {
    PlayQueueState(state::PlayQueueInfo),
}

#[derive(Debug, Clone)]
pub enum SinkMessage {
    PlayButton,
    PauseButton,
    LoadSong(path::PathBuf, Option<TrackPathOrPause>, f32),
    SetNextSong(TrackPathOrPause),
    SetVolume(f32),
    Close,
}

#[derive(Debug, Clone)]
pub enum TrackPathOrPause {
    TrackPath(path::PathBuf),
    Pause,
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
