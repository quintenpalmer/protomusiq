use std::sync::mpsc;
use std::thread;
use std::time;

use crate::datastore::loader;
use crate::model;
use crate::shared;

use crate::services::{mpris, sink, tracker};

mod playback;

pub fn create_backend_with_client_and_callback(
    config_state: model::app::AppConfigState,
    loader: loader::Loader,
) -> (
    shared::Client<shared::GUIToBackendMessage>,
    shared::Callback<shared::BackendToGUIMessage>,
) {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    let (callback_from_backend, callback_to_client) = mpsc::channel();

    thread::spawn(move || {
        run_forever(
            config_state,
            loader,
            recv_for_backend,
            callback_from_backend,
        )
    });

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
    config_state: model::app::AppConfigState,
    loader: loader::Loader,
    gui_rx: mpsc::Receiver<shared::GUIToBackendMessage>,
    gui_callback: mpsc::Sender<shared::BackendToGUIMessage>,
) {
    println!("MULTI-BACKEND:\tstarting to listen...");

    let mut tracked_state = TrackedState::new();

    let mut play_queue = shared::PlayQueueInfo::new();

    let (sink_client, sink_callback) = sink::create_backend_with_client_and_callback();

    let (mpris_client, mpris_callback) = mpris::create_backend_with_client_and_callback();

    let tracker_client = tracker::create_backend_with_client(config_state, loader);

    loop {
        if tracked_state.gui_closed && tracked_state.mpris_closed && tracked_state.sink_closed {
            break;
        }
        match gui_rx.try_recv() {
            Ok(gui_msg) => match gui_msg {
                shared::GUIToBackendMessage::BackendPlayback(to_playback_msg) => {
                    playback::handle_playback_request(
                        &mut play_queue,
                        sink_client.clone(),
                        mpris_client.clone(),
                        tracker_client.clone(),
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

                playback::handle_playback_request(
                    &mut play_queue,
                    sink_client.clone(),
                    mpris_client.clone(),
                    tracker_client.clone(),
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
                        playback::handle_playback_request(
                            &mut play_queue,
                            sink_client.clone(),
                            mpris_client.clone(),
                            tracker_client.clone(),
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
