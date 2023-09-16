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
    playing: bool,
    sink_closed: bool,
    mpris_closed: bool,
    gui_closed: bool,
}

impl TrackedState {
    fn new() -> Self {
        TrackedState {
            playing: false,
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

    let (sink_client, sink_callback) = sink::create_backend_with_client_and_callback();

    let (mpris_client, mpris_callback) = mpris::create_backend_with_client_and_callback();

    loop {
        if tracked_state.gui_closed && tracked_state.mpris_closed && tracked_state.sink_closed {
            break;
        }
        match gui_rx.try_recv() {
            Ok(gui_msg) => match gui_msg {
                shared::GUIToBackendMessage::ToSink(sink_msg) => {
                    let maybe_mpris_msg = match sink_msg {
                        shared::SinkMessage::PlayButton => Some(shared::MprisMessage::SetPlaying),
                        shared::SinkMessage::PauseButton => Some(shared::MprisMessage::SetPaused),
                        shared::SinkMessage::LoadSong(ref _file_path, ref _volume) => None,
                        shared::SinkMessage::SetVolume(ref _volume) => None,
                        shared::SinkMessage::Close => Some(shared::MprisMessage::Close),
                    };

                    match sink_msg {
                        shared::SinkMessage::PlayButton => tracked_state.playing = true,
                        shared::SinkMessage::PauseButton => tracked_state.playing = false,
                        shared::SinkMessage::LoadSong(ref _file_path, ref _volume) => {
                            tracked_state.playing = true
                        }
                        shared::SinkMessage::SetVolume(ref _volume) => (),
                        shared::SinkMessage::Close => {
                            tracked_state.gui_closed = true;
                            tracked_state.sink_closed = true;
                            tracked_state.mpris_closed = true;
                        }
                    };
                    sink_client.send(sink_msg).unwrap();
                    match maybe_mpris_msg {
                        Some(mpris_msg) => mpris_client.send(mpris_msg).unwrap(),
                        None => (),
                    };
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
                let maybe_sink_msg = match mpris_message {
                    shared::MprisCallbackMessage::PlayPause => match tracked_state.playing {
                        true => Some(shared::SinkMessage::PauseButton),
                        false => Some(shared::SinkMessage::PlayButton),
                    },
                    shared::MprisCallbackMessage::Play => Some(shared::SinkMessage::PlayButton),
                    shared::MprisCallbackMessage::Pause => Some(shared::SinkMessage::PauseButton),
                    shared::MprisCallbackMessage::Prev => None,
                    shared::MprisCallbackMessage::Next => None,
                };

                match mpris_message {
                    shared::MprisCallbackMessage::PlayPause => {
                        tracked_state.playing = !tracked_state.playing
                    }
                    shared::MprisCallbackMessage::Play => tracked_state.playing = true,
                    shared::MprisCallbackMessage::Pause => tracked_state.playing = false,
                    shared::MprisCallbackMessage::Prev => (),
                    shared::MprisCallbackMessage::Next => (),
                };

                let _ = gui_callback.send(shared::BackendToGUIMessage::MprisReports(mpris_message));
                match maybe_sink_msg {
                    Some(sink_msg) => {
                        let _ = sink_client.send(sink_msg);
                    }
                    None => (),
                };
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
                    shared::SinkCallbackMessage::Playing => (),
                    shared::SinkCallbackMessage::Paused => (),
                    shared::SinkCallbackMessage::SecondElapsed => (),
                    shared::SinkCallbackMessage::SongEnded => (),
                };
                let _ = gui_callback.send(shared::BackendToGUIMessage::SinkReports(sink_message));
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
