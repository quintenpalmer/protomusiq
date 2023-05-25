use iced::Command;

use crate::shared;

use super::message::{self, Message};
use super::state::AppState;

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
            common::sink_sender(
                app.player_info.sink_message_sender.clone(),
                shared::SinkMessage::SetVolume(app.player_info.current_volume),
            )
            .send_message()
        },
        Message::ErrorResponse,
    )
}
