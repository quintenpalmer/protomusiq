use crate::model;

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
pub enum CurrentPlayback {
    Track(CurrentTrackPlayback),
    PauseBreak,
}

impl CurrentPlayback {
    pub fn to_play_queue_entry(self) -> PlayQueueEntry {
        match self {
            CurrentPlayback::Track(track) => {
                PlayQueueEntry::Track(PlayQueueTrack { track: track.track })
            }
            CurrentPlayback::PauseBreak => PlayQueueEntry::Action(PlayQueueAction::Pause),
        }
    }

    pub fn from_shared(shared_repr: PlayQueueEntry, current_second: u64) -> Self {
        match shared_repr {
            PlayQueueEntry::Track(t) => CurrentPlayback::Track(CurrentTrackPlayback {
                track: t.track,
                current_second,
            }),
            PlayQueueEntry::Action(PlayQueueAction::Pause) => CurrentPlayback::PauseBreak,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentTrackPlayback {
    pub track: model::AugmentedTrack,
    pub current_second: u64,
}
