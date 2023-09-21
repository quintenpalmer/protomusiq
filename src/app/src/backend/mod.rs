use std::sync::mpsc;
use std::thread;
use std::time;

use crate::shared;

use crate::services::{mpris, sink};

pub fn create_backend_with_client_and_callback() -> (
    shared::Client<shared::GUIToBackendMessage>,
    shared::Callback<shared::BackendToGUIMessage>,
) {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    let (callback_from_backend, callback_to_client) = mpsc::channel();

    thread::spawn(move || run_forever(recv_for_backend, callback_from_backend));

    (
        shared::Client::new(sender_for_client),
        shared::Callback::new(callback_to_client),
    )
}

struct TrackedState {
    sink_closed: bool,
    mpris_closed: bool,
    gui_closed: bool,
}

impl TrackedState {
    fn new() -> Self {
        TrackedState {
            sink_closed: false,
            mpris_closed: false,
            gui_closed: false,
        }
    }
}

pub fn run_forever(
    gui_rx: mpsc::Receiver<shared::GUIToBackendMessage>,
    gui_callback: mpsc::Sender<shared::BackendToGUIMessage>,
) {
    println!("MULTI-BACKEND:\tstarting to listen...");

    let mut tracked_state = TrackedState::new();

    let mut play_queue = shared::PlayQueueInfo::new();

    let (sink_client, sink_callback) = sink::create_backend_with_client_and_callback();

    let (mpris_client, mpris_callback) = mpris::create_backend_with_client_and_callback();

    loop {
        if tracked_state.gui_closed && tracked_state.mpris_closed && tracked_state.sink_closed {
            break;
        }
        match gui_rx.try_recv() {
            Ok(gui_msg) => match gui_msg {
                shared::GUIToBackendMessage::ToSink(to_playback_msg) => {
                    handle_playback_request(
                        &mut play_queue,
                        sink_client.clone(),
                        mpris_client.clone(),
                        to_playback_msg,
                    );

                    let _ = gui_callback.send(shared::BackendToGUIMessage::PlayQueueState(
                        play_queue.clone(),
                    ));
                }
            },
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("recv sees that all clients have closed");
                tracked_state.gui_closed = true;
            }
        }

        match mpris_callback.try_recv() {
            Ok(mpris_message) => {
                let playback_msg = match mpris_message {
                    shared::MprisCallbackMessage::PlayPause => {
                        if play_queue.playing {
                            shared::PlaybackRequest::Pause
                        } else {
                            shared::PlaybackRequest::Play
                        }
                    }
                    shared::MprisCallbackMessage::Play => shared::PlaybackRequest::Play,
                    shared::MprisCallbackMessage::Pause => shared::PlaybackRequest::Pause,
                    shared::MprisCallbackMessage::Prev => shared::PlaybackRequest::Prev,
                    shared::MprisCallbackMessage::Next => shared::PlaybackRequest::Next,
                };

                handle_playback_request(
                    &mut play_queue,
                    sink_client.clone(),
                    mpris_client.clone(),
                    playback_msg,
                );

                let _ = gui_callback.send(shared::BackendToGUIMessage::PlayQueueState(
                    play_queue.clone(),
                ));
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("recv sees that all clients have closed");
                tracked_state.mpris_closed = true;
            }
        }

        match sink_callback.try_recv() {
            Ok(sink_message) => {
                match sink_message {
                    shared::SinkCallbackMessage::Playing => play_queue.playing = true,
                    shared::SinkCallbackMessage::Paused => play_queue.playing = false,
                    shared::SinkCallbackMessage::SecondElapsed => {
                        play_queue.current_second += 1;
                    }
                    shared::SinkCallbackMessage::SongEnded => {
                        handle_playback_request(
                            &mut play_queue,
                            sink_client.clone(),
                            mpris_client.clone(),
                            shared::PlaybackRequest::Next,
                        );
                    }
                };
                let _ = gui_callback.send(shared::BackendToGUIMessage::PlayQueueState(
                    play_queue.clone(),
                ));
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("recv sees that all clients have closed");
                tracked_state.sink_closed = true;
            }
        }

        thread::sleep(time::Duration::from_millis(50));
    }

    println!("MULTI-BACKEND:\tdone listening");
}

fn handle_playback_request(
    play_queue: &mut shared::PlayQueueInfo,
    sink_client: shared::Client<shared::SinkMessage>,
    mpris_client: shared::Client<shared::MprisMessage>,
    internal: shared::PlaybackRequest,
) {
    println!("GUI:\thandling internal: {:?}", internal);
    match internal {
        shared::PlaybackRequest::LoadCurrentSong => match play_queue.current_playback {
            Some(ref outer_current_playback) => match outer_current_playback {
                shared::PlayQueueEntry::Track(ref current_playback) => {
                    let _ = sink_client
                        .send(shared::SinkMessage::LoadSong(
                            current_playback.track.metadata.path.clone(),
                            play_queue.current_volume,
                        ))
                        .unwrap();
                    let _ = mpris_client.send(shared::MprisMessage::SetMetadata(
                        current_playback.track.metadata.album_artist.clone(),
                        current_playback.track.metadata.title.clone(),
                    ));
                }
                shared::PlayQueueEntry::Action(shared::PlayQueueAction::Pause) => {
                    play_queue.playing = false;
                    handle_playback_request(
                        play_queue,
                        sink_client,
                        mpris_client,
                        shared::PlaybackRequest::Pause,
                    );
                }
            },
            None => println!("Nothing to load for current song"),
        },
        shared::PlaybackRequest::PlaySongs(tracks) => handle_playback_request(
            play_queue,
            sink_client,
            mpris_client,
            shared::PlaybackRequest::InsertSongs(tracks, true),
        ),
        shared::PlaybackRequest::AppendSongs(tracks, load_next) => {
            let mut new_songs_to_queue = Vec::new();
            for iter_track in tracks.into_iter() {
                new_songs_to_queue.push(shared::PlayQueueEntry::Track(shared::PlayQueueTrack {
                    track: iter_track,
                }));
            }
            play_queue.play_queue.append(&mut new_songs_to_queue);
            if load_next {
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    shared::PlaybackRequest::Next,
                );
            } else {
                // Nothing else to run if not loading next
            }
        }
        shared::PlaybackRequest::InsertSongs(tracks, load_next) => {
            let mut new_songs_to_queue = Vec::new();
            for iter_track in tracks.into_iter() {
                new_songs_to_queue.push(shared::PlayQueueEntry::Track(shared::PlayQueueTrack {
                    track: iter_track,
                }));
            }
            new_songs_to_queue.append(&mut play_queue.play_queue);
            play_queue.play_queue = new_songs_to_queue;

            if load_next {
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    shared::PlaybackRequest::Next,
                );
            } else {
                // Nothing else to run if not loading next
            }
        }
        shared::PlaybackRequest::Prev => {
            if play_queue.play_history.len() > 0 {
                match play_queue.current_playback {
                    Some(ref current_playback) => {
                        let mut new_play_queue = vec![current_playback.clone()];
                        new_play_queue.append(&mut play_queue.play_queue);
                        play_queue.play_queue = new_play_queue;
                    }
                    None => (),
                };
                let track = play_queue.play_history.pop().unwrap();
                play_queue.current_playback = Some(match track {
                    shared::PlayQueueEntry::Track(ref t) => {
                        shared::PlayQueueEntry::Track(shared::PlayQueueTrack {
                            track: t.track.clone(),
                        })
                    }
                    shared::PlayQueueEntry::Action(shared::PlayQueueAction::Pause) => {
                        shared::PlayQueueEntry::Action(shared::PlayQueueAction::Pause)
                    }
                });
                play_queue.current_playback = Some(track.clone());
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    shared::PlaybackRequest::LoadCurrentSong,
                );
            } else {
                // Nothing else to run if not loading next
            }
        }
        shared::PlaybackRequest::Next => {
            if play_queue.play_queue.len() > 0 {
                match play_queue.current_playback {
                    Some(ref current_playback) => {
                        play_queue.play_history.push(current_playback.clone())
                    }
                    None => (),
                };

                let track = play_queue.play_queue.remove(0);
                //play_queue.current_playback = Some(state::CurrentPlayback::from_entry_zeroed(&track));
                play_queue.current_second = 0;
                play_queue.current_playback = Some(track.clone());
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    shared::PlaybackRequest::LoadCurrentSong,
                );
            } else {
                match play_queue.current_playback {
                    Some(ref current_playback) => {
                        play_queue.play_history.push(current_playback.clone())
                    }
                    None => (),
                };
                play_queue.current_playback = None;
                let _ = mpris_client.send(shared::MprisMessage::SetStopped);
            }
        }
        shared::PlaybackRequest::Play => {
            play_queue.playing = true;
            let _ = sink_client.send(shared::SinkMessage::PlayButton);
            let _ = mpris_client.send(shared::MprisMessage::SetPlaying);
        }
        shared::PlaybackRequest::Pause => {
            play_queue.playing = false;
            let _ = sink_client.send(shared::SinkMessage::PauseButton);
            let _ = mpris_client.send(shared::MprisMessage::SetPaused);
        }
        shared::PlaybackRequest::InsertPause => {
            let mut new_songs_to_queue = vec![shared::PlayQueueEntry::Action(
                shared::PlayQueueAction::Pause,
            )];
            new_songs_to_queue.append(&mut play_queue.play_queue);
            play_queue.play_queue = new_songs_to_queue;
        }
        shared::PlaybackRequest::SetVolume(new_volume) => {
            let _ = sink_client.send(shared::SinkMessage::SetVolume(new_volume));
        }
        shared::PlaybackRequest::Close => {
            let _ = sink_client.send(shared::SinkMessage::Close);
            let _ = mpris_client.send(shared::MprisMessage::Close);
        }
    }
}
