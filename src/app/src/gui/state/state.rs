use std::cell::RefCell;

use iced::{button, scrollable};

use crate::shared;

use crate::datastore::staticassets::embedded;
use crate::model;

use super::super::message;
use super::page;

/// Top Level Application
pub enum App {
    Loading,
    Loaded(Loaded),
}

/// State for the App, once it has finished processing all data it needs
/// to provide its user experience
pub struct Loaded {
    pub gui: AppGuiState,
    pub rest: AppState,
}

pub struct AppGuiState {
    pub back_button: button::State,
    pub home_breadcrumb: button::State,
    pub search_button: button::State,
    pub config_button: button::State,
    pub close_button: button::State,
}

pub struct AppState {
    pub current_page: page::Page,
    pub library: model::LibraryState,
    pub player_info: PlayerInfo,
    pub play_queue_info: PlayQueueInfo,
    pub config: Config,
    pub should_close: bool,

    pub page_back_history: Vec<message::NavMessage>,
    pub page_current_history: message::NavMessage,
    pub page_forward_history: Vec<message::NavMessage>,

    pub app_images: embedded::AppImages,

    pub error_messages: Vec<String>,
}

/// State for the Configuratino of the App
pub struct Config {
    pub gui: AppConfigGuiState,
    pub rest: model::AppConfigState,
}

pub struct AppConfigGuiState {}

/// State for the Play Queue (and Current Track and Play History)
pub struct PlayQueueInfo {
    pub gui: PlayQueueGuiState,
    pub rest: PlayQueueInfoState,
}

pub struct PlayQueueGuiState {
    pub hide_play_queue: button::State,
    pub play_queue_page_button: button::State,
    pub play_queue_scroll: scrollable::State,
    pub track_info: PlayQueueTrackGuiState,
}

pub struct PlayQueueTrackGuiState {
    pub play_history: Vec<PlayQueueGuiEntry>,
    pub play_queue: Vec<PlayQueueGuiEntry>,
}

pub struct PlayQueueGuiEntry {
    pub remove_me_button: button::State,
}

pub struct PlayQueueInfoState {
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

#[derive(Debug, Clone)]
pub struct PlayQueueTrack {
    pub track: model::AugmentedTrack,
}

#[derive(Debug, Clone)]
pub enum PlayQueueAction {
    Pause,
}

impl PlayQueueEntry {
    pub fn from_playback(playback: &CurrentPlayback) -> Self {
        match playback {
            CurrentPlayback::Track(ref current_playback) => PlayQueueEntry::Track(PlayQueueTrack{track: current_playback.track.clone()}),
            CurrentPlayback::PauseBreak => PlayQueueEntry::Action(PlayQueueAction::Pause),
        }
    }

    pub fn to_playback_zeroed(&self) -> CurrentPlayback {
        match self {
            PlayQueueEntry::Track(ref track) => CurrentPlayback::Track(
                CurrentTrackPlayback {
                    track: track.track.clone(),
                    current_second: 0,
                }
            ),
            PlayQueueEntry::Action(PlayQueueAction::Pause) => CurrentPlayback::PauseBreak,
        }
    }
}

/// State for the Playback, its services, and its controls
pub struct PlayerInfo {
    pub gui: PlayerInfoGuiState,
    pub rest: PlayerInfoState,
}

pub struct PlayerInfoGuiState {
    pub play_button: button::State,
    pub pause_button: button::State,
    pub prev_button: button::State,
    pub next_button: button::State,
    pub pause_next_button: button::State,

    pub volume_zero_button: button::State,
    pub volume_up_button: button::State,
    pub volume_down_button: button::State,
    pub volume_max_button: button::State,
}

pub struct PlayerInfoState {
    pub playing: bool,
    pub current_volume: f32,
    pub current_playback: Option<CurrentPlayback>,

    pub sink_message_sender: shared::Client<shared::SinkMessage>,
    pub sink_callback_recv: RefCell<Option<shared::Callback<shared::SinkCallbackMessage>>>,
    pub mpris_message_sender: shared::Client<shared::MprisMessage>,
    pub mpris_callback_recv: RefCell<Option<shared::Callback<shared::MprisCallbackMessage>>>,
    pub tracker_message_sender: shared::Client<shared::TrackerMessage>,
}

impl PlayerInfoState {
    pub fn get_maybe_current_playback_track(&self) -> Option<&model::AugmentedTrack> {
        match self.current_playback {
            Some(ref o) => match o {
                CurrentPlayback::Track(ref v) => Some(&v.track),
                _ => None,
            }
            None => None,
        }
    }
}

pub enum CurrentPlayback {
    Track(CurrentTrackPlayback),
    PauseBreak,
}

pub struct CurrentTrackPlayback {
    pub track: model::AugmentedTrack,
    pub current_second: u64,
}

impl CurrentPlayback {
    pub fn from_entry_zeroed(playback: &PlayQueueEntry) -> Self {
        playback.to_playback_zeroed()
    }

    #[allow(unused)]
    pub fn to_entry(&self) -> PlayQueueEntry{
        PlayQueueEntry::from_playback(self)
    }
}
