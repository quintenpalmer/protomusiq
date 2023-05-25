use iced::Command;

use crate::shared;

use super::init;
use super::message::{self, Message};
use super::state::{self, App, AppState};

mod action;
mod common;
mod nav;
mod volume;

pub fn update_from_loading_state(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::Action(message::Action::LoadEverything) => {
            *app = init::initialize_everything();
            Command::none()
        }
        _ => {
            println!("cannot process {:?} before we are loaded", message);
            Command::none()
        }
    }
}

pub fn update_state(app: &mut AppState, message: Message) -> Command<Message> {
    println!(
        "GUI:\tupdating with {:?} ({})",
        message,
        app.page_back_history
            .iter()
            .fold("".to_string(), |total, current| {
                format!("{:?}, {}", current, total)
            })
    );

    match message {
        Message::Action(action) => action::handle_action(app, action),
        Message::Nav(nav_message) => {
            app.page_back_history.push(app.page_current_history.clone());
            app.page_current_history = nav_message.clone();
            nav::handle_nav(app, nav_message)
        }
        Message::HistoryNav => match app.page_back_history.pop() {
            Some(history_message) => {
                let old_current = app.page_current_history.clone();
                app.page_current_history = history_message.clone();
                app.page_forward_history.insert(0, old_current);
                nav::handle_nav(app, history_message)
            }
            None => Command::none(),
        },
        Message::PlaybackRequest(internal) => {
            println!("GUI:\thandling internal: {:?}", internal);
            match internal {
                message::PlaybackRequest::LoadCurrentSong => {
                    match app.player_info.current_playback {
                        Some(ref outer_current_playback) => match outer_current_playback {
                            state::CurrentPlayback::Track(ref current_playback) => {
                                app.player_info.playing = true;
                                Command::batch(vec![
                                    Command::perform(
                                        common::mpris_sender(
                                            app.player_info.mpris_message_sender.clone(),
                                            shared::MprisMessage::SetMetadata(
                                                current_playback
                                                    .track
                                                    .metadata
                                                    .album_artist
                                                    .clone(),
                                                current_playback.track.metadata.title.clone(),
                                            ),
                                        )
                                        .send_message(),
                                        Message::ErrorResponse,
                                    ),
                                    Command::perform(
                                        {
                                            common::sink_sender(
                                                app.player_info.sink_message_sender.clone(),
                                                shared::SinkMessage::LoadSong(
                                                    current_playback.track.metadata.path.clone(),
                                                    app.player_info.current_volume,
                                                ),
                                            )
                                            .send_message()
                                        },
                                        Message::ErrorResponse,
                                    ),
                                    Command::perform(
                                        common::tracker_sender(
                                            app.player_info.tracker_message_sender.clone(),
                                            shared::TrackerMessage::SongStarted(
                                                current_playback.track.clone(),
                                            ),
                                        )
                                        .send_message(),
                                        Message::ErrorResponse,
                                    ),
                                ])
                            }
                            state::CurrentPlayback::PauseBreak => {
                                //app.player_info.rest.playing = false;
                                message::message_command(Message::PlaybackRequest(
                                    message::PlaybackRequest::Pause,
                                ))
                            }
                        },
                        None => Command::none(),
                    }
                }
                message::PlaybackRequest::PlaySongs(tracks) => message::message_command(
                    Message::PlaybackRequest(message::PlaybackRequest::InsertSongs(tracks, true)),
                ),
                message::PlaybackRequest::AppendSongs(tracks, load_next) => {
                    let mut new_songs_to_queue = Vec::new();
                    for iter_track in tracks.into_iter() {
                        new_songs_to_queue.push(state::PlayQueueEntry::Track(
                            state::PlayQueueTrack { track: iter_track },
                        ));
                    }
                    app.play_queue_info
                        .play_queue
                        .append(&mut new_songs_to_queue);
                    if load_next {
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::Next,
                        ))
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::InsertSongs(tracks, load_next) => {
                    let mut new_songs_to_queue = Vec::new();
                    for iter_track in tracks.into_iter() {
                        new_songs_to_queue.push(state::PlayQueueEntry::Track(
                            state::PlayQueueTrack { track: iter_track },
                        ));
                    }
                    new_songs_to_queue.append(&mut app.play_queue_info.play_queue);
                    app.play_queue_info.play_queue = new_songs_to_queue;

                    if load_next {
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::Next,
                        ))
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::Prev => {
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
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::LoadCurrentSong,
                        ))
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::Next => {
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
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::LoadCurrentSong,
                        ))
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
                        Command::perform(
                            common::mpris_sender(
                                app.player_info.mpris_message_sender.clone(),
                                shared::MprisMessage::SetStopped,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        )
                    }
                }
                message::PlaybackRequest::Play => {
                    app.player_info.playing = true;
                    Command::batch(vec![
                        Command::perform(
                            common::mpris_sender(
                                app.player_info.mpris_message_sender.clone(),
                                shared::MprisMessage::SetPlaying,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                        Command::perform(
                            common::sink_sender(
                                app.player_info.sink_message_sender.clone(),
                                shared::SinkMessage::PlayButton,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                    ])
                }
                message::PlaybackRequest::Pause => {
                    app.player_info.playing = false;
                    Command::batch(vec![
                        Command::perform(
                            common::mpris_sender(
                                app.player_info.mpris_message_sender.clone(),
                                shared::MprisMessage::SetPaused,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                        Command::perform(
                            common::sink_sender(
                                app.player_info.sink_message_sender.clone(),
                                shared::SinkMessage::PauseButton,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                    ])
                }
                message::PlaybackRequest::InsertPause => {
                    let mut new_songs_to_queue =
                        vec![state::PlayQueueEntry::Action(state::PlayQueueAction::Pause)];
                    new_songs_to_queue.append(&mut app.play_queue_info.play_queue);
                    app.play_queue_info.play_queue = new_songs_to_queue;

                    Command::none()
                }
            }
        }
        Message::SinkCallback(callback) => match callback {
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
        },
        Message::ErrorResponse(resp) => {
            match resp {
                Ok(()) => (),
                Err(e) => println!("error from calling out to subservice: {:?}", e),
            };
            Command::none()
        }
        Message::MprisCallback(callback) => message::message_command(match callback {
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
        }),
    }
}
