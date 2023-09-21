use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::{self, AppState};

use super::common;

pub fn handle_set_play_queue(
    app: &mut AppState,
    new_play_queue: shared::PlayQueueInfo,
) -> Command<message::Message> {
    app.play_queue_info.play_history = new_play_queue
        .play_history
        .into_iter()
        .map(|x| state::PlayQueueEntry::from_shared(x))
        .collect();
    app.play_queue_info.current_playback = new_play_queue
        .current_playback
        .clone()
        .map(|x| state::PlayQueueEntry::from_shared(x));
    app.play_queue_info.play_queue = new_play_queue
        .play_queue
        .into_iter()
        .map(|x| state::PlayQueueEntry::from_shared(x))
        .collect();
    let current_second = new_play_queue.current_second;
    app.player_info.current_playback = new_play_queue
        .current_playback
        .map(|x| state::CurrentPlayback::from_shared(x, current_second));
    app.player_info.playing = new_play_queue.playing;

    Command::none()
}

pub fn handle_playback_request(
    app: &mut AppState,
    playback_request: shared::PlaybackRequest,
) -> Command<message::Message> {
    Command::perform(
        common::backend_sender(
            app.player_info.backend_message_sender.clone(),
            shared::GUIToBackendMessage::ToSink(playback_request),
        )
        .send_message(),
        Message::ErrorResponse,
    )
}
