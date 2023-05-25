use iced::Command;

use crate::shared;

use super::message::{self, Message};
use super::state::{self, AppState};

pub fn handle_sink_callback(
    app: &mut AppState,
    callback: shared::SinkCallbackMessage,
) -> Command<message::Message> {
    match callback {
        shared::SinkCallbackMessage::SongEnded => {
            message::message_command(Message::PlaybackRequest(message::PlaybackRequest::Next))
        }
        shared::SinkCallbackMessage::SecondElapsed => {
            match app.player_info.current_playback {
                Some(ref mut outer_current_playback) => match outer_current_playback {
                    state::CurrentPlayback::Track(ref mut current_playback) => {
                        current_playback.current_second += 1
                    }
                    _ => println!("Hmmm, songs are playing back while on a pause break?"),
                },
                None => (),
            };
            Command::none()
        }
        shared::SinkCallbackMessage::Paused => Command::none(),
        shared::SinkCallbackMessage::Playing => Command::none(),
    }
}
