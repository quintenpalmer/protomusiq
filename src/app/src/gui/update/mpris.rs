use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::AppState;

use super::loaded;

pub fn handle_mpris_callback(
    app: &mut AppState,
    callback: shared::MprisCallbackMessage,
) -> Command<message::Message> {
    let followup_message = match callback {
        shared::MprisCallbackMessage::PlayPause => {
            if app.player_info.playing {
                Message::PlaybackRequest(shared::PlaybackRequest::Pause)
            } else {
                Message::PlaybackRequest(shared::PlaybackRequest::Play)
            }
        }
        shared::MprisCallbackMessage::Play => {
            Message::PlaybackRequest(shared::PlaybackRequest::Play)
        }
        shared::MprisCallbackMessage::Pause => {
            Message::PlaybackRequest(shared::PlaybackRequest::Pause)
        }
        shared::MprisCallbackMessage::Prev => {
            Message::PlaybackRequest(shared::PlaybackRequest::Prev)
        }
        shared::MprisCallbackMessage::Next => {
            Message::PlaybackRequest(shared::PlaybackRequest::Next)
        }
    };

    loaded::update_state(app, followup_message)
}
