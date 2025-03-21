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
    pub page_state: PageState,

    pub action_state: ActionState,
    pub cross_page_display_info: CrossPageDisplayInfo,

    pub config: musiqcore::model::app::AppConfigState,

    pub library: model::LibraryState,
    pub video_library: model::VideoLibraryState,
    pub show_library: musiqcore::model::shows::ShowLibraryState,
    pub game_library: musiqcore::model::gl::GameLibraryState,

    pub player_info: PlayerInfo,

    pub app_images: embedded::AppImages,
    pub messages: Vec<MessageInfo>,

    // TODO either use this or drop it
    #[allow(unused)]
    pub should_close: bool,
}

/// All Page related state information
pub struct PageState {
    pub current_page: page::Page,
    pub page_back_history: Vec<message::NavMessage>,
    pub page_current_history: message::NavMessage,
    pub page_forward_history: Vec<message::NavMessage>,
}

/// Whether to show in a maximally fullscreen layout
pub struct CrossPageDisplayInfo {
    pub fullscreen_display: bool,
}

/// A Single Message
pub struct MessageInfo {
    pub notification_type: message::NotificationAction,
}

/// State for the Play Queue (and Current Track and Play History)
pub struct PlayQueueInfo {
    pub play_queue_visible: bool,
    pub play_history: Vec<shared::PlayQueueEntry>,
    pub current_playback: Option<shared::CurrentPlayback>,
    pub play_queue: Vec<shared::PlayQueueEntry>,
}

pub struct ActionState {
    pub group_buttons_shuffle: bool,
}

/// State for the Playback, its services, and its controls
pub struct PlayerInfo {
    pub playing: bool,
    pub current_volume: f32,
    pub play_queue_info: PlayQueueInfo,

    pub backend_message_sender: shared::Client<shared::GUIToBackendMessage>,
    pub backend_callback_recv: RefCell<Option<shared::Callback<shared::BackendToGUIMessage>>>,
}

impl PlayerInfo {
    pub fn get_maybe_current_playback_track(&self) -> Option<&model::AugmentedTrack> {
        match self.play_queue_info.current_playback {
            Some(ref o) => match o {
                shared::CurrentPlayback::Track(ref v) => Some(&v.track),
                _ => None,
            },
            None => None,
        }
    }
}
