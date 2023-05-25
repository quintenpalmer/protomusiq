use iced::Command;

use crate::shared;

use super::message::{self, Message};
use super::state::AppState;

pub fn handle_mpris_callback(
    app: &mut AppState,
    callback: shared::MprisCallbackMessage,
) -> Command<message::Message> {
    message::message_command(match callback {
        shared::MprisCallbackMessage::PlayPause => {
            if app.player_info.playing {
                Message::PlaybackRequest(message::PlaybackRequest::Pause)
            } else {
                Message::PlaybackRequest(message::PlaybackRequest::Play)
            }
        }
        shared::MprisCallbackMessage::Play => {
            Message::PlaybackRequest(message::PlaybackRequest::Play)
        }
        shared::MprisCallbackMessage::Pause => {
            Message::PlaybackRequest(message::PlaybackRequest::Pause)
        }
        shared::MprisCallbackMessage::Prev => {
            Message::PlaybackRequest(message::PlaybackRequest::Prev)
        }
        shared::MprisCallbackMessage::Next => {
            Message::PlaybackRequest(message::PlaybackRequest::Next)
        }
    })
}
