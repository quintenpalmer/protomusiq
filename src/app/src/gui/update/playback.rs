use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::{self, AppState};

use super::common;
use super::loaded;

pub fn handle_playback_request(
    app: &mut AppState,
    internal: shared::PlaybackRequest,
) -> Command<message::Message> {
    println!("GUI:\thandling internal: {:?}", internal);
    match internal {
        shared::PlaybackRequest::LoadCurrentSong => {
            match app.player_info.current_playback {
                Some(ref outer_current_playback) => match outer_current_playback {
                    state::CurrentPlayback::Track(ref current_playback) => {
                        app.player_info.playing = true;
                        Command::perform(
                            {
                                common::backend_sender(
                                    app.player_info.backend_message_sender.clone(),
                                    shared::GUIToBackendMessage::ToSink(
                                        shared::SinkMessage::LoadSong(
                                            current_playback.track.metadata.path.clone(),
                                            app.player_info.current_volume,
                                        ),
                                    ),
                                )
                                .send_message()
                            },
                            Message::ErrorResponse,
                        )
                    }
                    state::CurrentPlayback::PauseBreak => {
                        //app.player_info.rest.playing = false;
                        loaded::update_state(
                            app,
                            Message::PlaybackRequest(shared::PlaybackRequest::Pause),
                        )
                    }
                },
                None => Command::none(),
            }
        }
        shared::PlaybackRequest::PlaySongs(tracks) => loaded::update_state(
            app,
            Message::PlaybackRequest(shared::PlaybackRequest::InsertSongs(tracks, true)),
        ),
        shared::PlaybackRequest::AppendSongs(tracks, load_next) => {
            let mut new_songs_to_queue = Vec::new();
            for iter_track in tracks.into_iter() {
                new_songs_to_queue.push(state::PlayQueueEntry::Track(state::PlayQueueTrack {
                    track: iter_track,
                }));
            }
            app.play_queue_info
                .play_queue
                .append(&mut new_songs_to_queue);
            if load_next {
                loaded::update_state(app, Message::PlaybackRequest(shared::PlaybackRequest::Next))
            } else {
                Command::none()
            }
        }
        shared::PlaybackRequest::InsertSongs(tracks, load_next) => {
            let mut new_songs_to_queue = Vec::new();
            for iter_track in tracks.into_iter() {
                new_songs_to_queue.push(state::PlayQueueEntry::Track(state::PlayQueueTrack {
                    track: iter_track,
                }));
            }
            new_songs_to_queue.append(&mut app.play_queue_info.play_queue);
            app.play_queue_info.play_queue = new_songs_to_queue;

            if load_next {
                loaded::update_state(app, Message::PlaybackRequest(shared::PlaybackRequest::Next))
            } else {
                Command::none()
            }
        }
        shared::PlaybackRequest::Prev => {
            if app.play_queue_info.play_history.len() > 0 {
                match app.player_info.current_playback {
                    Some(ref current_playback) => {
                        let mut new_play_queue =
                            vec![state::PlayQueueEntry::from_playback(current_playback)];
                        new_play_queue.append(&mut app.play_queue_info.play_queue);
                        app.play_queue_info.play_queue = new_play_queue;
                    }
                    None => (),
                };
                let track = app.play_queue_info.play_history.pop().unwrap();
                app.player_info.current_playback = Some(match track {
                    state::PlayQueueEntry::Track(ref t) => {
                        state::CurrentPlayback::Track(state::CurrentTrackPlayback {
                            track: t.track.clone(),
                            current_second: 0,
                        })
                    }
                    state::PlayQueueEntry::Action(state::PlayQueueAction::Pause) => {
                        state::CurrentPlayback::PauseBreak
                    }
                });
                app.play_queue_info.current_playback = Some(track.clone());
                loaded::update_state(
                    app,
                    Message::PlaybackRequest(shared::PlaybackRequest::LoadCurrentSong),
                )
            } else {
                Command::none()
            }
        }
        shared::PlaybackRequest::Next => {
            if app.play_queue_info.play_queue.len() > 0 {
                match app.player_info.current_playback {
                    Some(ref current_playback) => app
                        .play_queue_info
                        .play_history
                        .push(state::PlayQueueEntry::from_playback(current_playback)),
                    None => (),
                };

                let track = app.play_queue_info.play_queue.remove(0);
                app.player_info.current_playback =
                    Some(state::CurrentPlayback::from_entry_zeroed(&track));
                app.play_queue_info.current_playback = Some(track.clone());
                loaded::update_state(
                    app,
                    Message::PlaybackRequest(shared::PlaybackRequest::LoadCurrentSong),
                )
            } else {
                match app.player_info.current_playback {
                    Some(ref current_playback) => app
                        .play_queue_info
                        .play_history
                        .push(state::PlayQueueEntry::from_playback(current_playback)),
                    None => (),
                };
                app.player_info.current_playback = None;
                app.play_queue_info.current_playback = None;
                Command::none()
            }
        }
        shared::PlaybackRequest::Play => {
            app.player_info.playing = true;
            Command::perform(
                common::backend_sender(
                    app.player_info.backend_message_sender.clone(),
                    shared::GUIToBackendMessage::ToSink(shared::SinkMessage::PlayButton),
                )
                .send_message(),
                Message::ErrorResponse,
            )
        }
        shared::PlaybackRequest::Pause => {
            app.player_info.playing = false;
            Command::perform(
                common::backend_sender(
                    app.player_info.backend_message_sender.clone(),
                    shared::GUIToBackendMessage::ToSink(shared::SinkMessage::PauseButton),
                )
                .send_message(),
                Message::ErrorResponse,
            )
        }
        shared::PlaybackRequest::InsertPause => {
            let mut new_songs_to_queue =
                vec![state::PlayQueueEntry::Action(state::PlayQueueAction::Pause)];
            new_songs_to_queue.append(&mut app.play_queue_info.play_queue);
            app.play_queue_info.play_queue = new_songs_to_queue;

            Command::none()
        }
    }
}
