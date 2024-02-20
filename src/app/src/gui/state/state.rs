use std::cell::RefCell;

use crate::shared;

use crate::datastore::staticassets::embedded;
use crate::model;

use super::super::message;
use super::page;

/// Top Level Application
pub enum App {
    Loading,
    Loaded(AppState),
}

/// State for the App, once it has finished processing all data it needs
/// to provide its user experience
pub struct AppState {
    pub current_page: page::Page,
    pub library: model::LibraryState,
    pub video_library: model::VideoLibraryState,
    pub action_state: ActionState,
    pub player_info: PlayerInfo,
    pub play_queue_info: PlayQueueInfo,
    pub messages: Vec<MessageInfo>,
    pub config: Config,
    pub should_close: bool,

    pub page_back_history: Vec<message::NavMessage>,
    pub page_current_history: message::NavMessage,
    pub page_forward_history: Vec<message::NavMessage>,

    pub app_images: embedded::AppImages,

    pub error_messages: Vec<String>,
}

/// A Single Message
pub struct MessageInfo {
    pub notification_type: message::NotificationAction,
}

/// State for the Configuratino of the App
pub struct Config {
    pub rest: model::app::AppConfigState,
}

/// State for the Play Queue (and Current Track and Play History)
pub struct PlayQueueInfo {
    pub play_queue_visible: bool,
    pub play_history: Vec<PlayQueueEntry>,
    pub current_playback: Option<PlayQueueEntry>,
    pub play_queue: Vec<PlayQueueEntry>,
}

#[derive(Debug, Clone)]
pub enum PlayQueueEntry {
    Track(PlayQueueTrack),
    Action(PlayQueueAction),
}

impl PlayQueueEntry {
    pub fn from_shared(shared_repr: shared::PlayQueueEntry) -> Self {
        match shared_repr {
            shared::PlayQueueEntry::Track(t) => {
                PlayQueueEntry::Track(PlayQueueTrack { track: t.track })
            }
            shared::PlayQueueEntry::Action(shared::PlayQueueAction::Pause) => {
                PlayQueueEntry::Action(PlayQueueAction::Pause)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayQueueTrack {
    pub track: model::AugmentedTrack,
}

#[derive(Debug, Clone)]
pub enum PlayQueueAction {
    Pause,
}

pub struct ActionState {
    pub group_buttons_shuffle: bool,
}

/// State for the Playback, its services, and its controls
pub struct PlayerInfo {
    pub playing: bool,
    pub current_volume: f32,
    pub current_playback: Option<CurrentPlayback>,

    pub backend_message_sender: shared::Client<shared::GUIToBackendMessage>,
    pub backend_callback_recv: RefCell<Option<shared::Callback<shared::BackendToGUIMessage>>>,
}

impl PlayerInfo {
    pub fn get_maybe_current_playback_track(&self) -> Option<&model::AugmentedTrack> {
        match self.current_playback {
            Some(ref o) => match o {
                CurrentPlayback::Track(ref v) => Some(&v.track),
                _ => None,
            },
            None => None,
        }
    }
}

pub enum CurrentPlayback {
    Track(CurrentTrackPlayback),
    PauseBreak,
}

impl CurrentPlayback {
    pub fn from_shared(shared_repr: shared::PlayQueueEntry, current_second: u64) -> Self {
        match shared_repr {
            shared::PlayQueueEntry::Track(t) => CurrentPlayback::Track(CurrentTrackPlayback {
                track: t.track,
                current_second,
            }),
            shared::PlayQueueEntry::Action(shared::PlayQueueAction::Pause) => {
                CurrentPlayback::PauseBreak
            }
        }
    }
}

pub struct CurrentTrackPlayback {
    pub track: model::AugmentedTrack,
    pub current_second: u64,
}
