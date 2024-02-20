use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::{self, AppState};

use super::action;
use super::common;

pub fn handle_set_play_queue(
    app: &mut AppState,
    new_play_queue: shared::PlayQueueInfo,
) -> Command<message::Message> {
    app.play_queue_info.play_history = new_play_queue
        .play_history
        .into_iter()
        .map(state::PlayQueueEntry::from_shared)
        .collect();
    app.play_queue_info.current_playback = new_play_queue
        .current_playback
        .clone()
        .map(state::PlayQueueEntry::from_shared);
    app.play_queue_info.play_queue = new_play_queue
        .play_queue
        .into_iter()
        .map(state::PlayQueueEntry::from_shared)
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
    let maybe_tracks = match playback_request {
        shared::PlaybackRequest::PlaySongs(ref tracks) => Some(tracks),
        shared::PlaybackRequest::InsertSongs(ref tracks, ref _play) => Some(tracks),
        shared::PlaybackRequest::AppendSongs(ref tracks, ref _play) => Some(tracks),
        _ => None,
    };

    match maybe_tracks {
        Some(tracks) => {
            for track in tracks.iter() {
                let _follow_up_empty_action = action::handle_action(
                    app,
                    message::Action::Notify(message::NotificationMessage::OnScreen(
                        message::NotificationAction::AddedToPlayQueue(track.metadata.title.clone()),
                    )),
                );
            }
        }
        None => (),
    };

    Command::perform(
        common::backend_sender(
            app.player_info.backend_message_sender.clone(),
            shared::GUIToBackendMessage::BackendPlayback(playback_request),
        )
        .send_message(),
        Message::ErrorResponse,
    )
}
