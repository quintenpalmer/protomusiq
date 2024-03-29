use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::AppState;

use super::common;

pub fn handle_volume_request(
    app: &mut AppState,
    volume_request: message::VolumeRequest,
) -> Command<Message> {
    match volume_request {
        message::VolumeRequest::Up(delta) => app.player_info.current_volume += delta,
        message::VolumeRequest::Down(delta) => app.player_info.current_volume -= delta,
        message::VolumeRequest::Set(new_volume) => app.player_info.current_volume = new_volume,
    };
    Command::perform(
        {
            common::backend_sender(
                app.player_info.backend_message_sender.clone(),
                shared::GUIToBackendMessage::BackendPlayback(shared::PlaybackRequest::SetVolume(
                    app.player_info.current_volume,
                )),
            )
            .send_message()
        },
        Message::ErrorResponse,
    )
}
