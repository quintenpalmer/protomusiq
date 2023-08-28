use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::shared;

const BLOCKING_TIMEOUT: u64 = 1000;

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

pub fn run_forever(
    rx: mpsc::Receiver<shared::GUIToBackendMessage>,
    callback: mpsc::Sender<shared::BackendToGUIMessage>,
) {
    println!("MULTI-BACKEND:\tstarting to listen...");

    let sink = SinkPlayback::new();

    sink.run_forever(rx, callback);

    println!("MULTI-BACKEND:\tdone listening");
}

pub struct SinkPlayback {
    sink: rodio::Sink,
    _stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    manual_sink_status: Option<bool>,
    time_elapsed: u64,
}

impl SinkPlayback {
    pub fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        SinkPlayback {
            _stream: stream,
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
            stream_handle: stream_handle,
            manual_sink_status: None,
            time_elapsed: 0,
        }
    }

    pub fn run_forever(
        mut self,
        rx: mpsc::Receiver<shared::GUIToBackendMessage>,
        callback: mpsc::Sender<shared::BackendToGUIMessage>,
    ) {
        loop {
            match self.manual_sink_status {
                Some(true) => {
                    match rx.recv_timeout(time::Duration::from_millis(BLOCKING_TIMEOUT)) {
                        Ok(shared::GUIToBackendMessage::ToSink(msg)) => {
                            if !self.handle_msg(msg, &callback) {
                                break;
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => self.handle_timeout(&callback),
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            println!("recv sees that all clients have closed");
                            break;
                        }
                    }
                }
                _ => {
                    println!("SINK:\tno playing media so waiting forever on recv");
                    match rx.recv() {
                        Ok(shared::GUIToBackendMessage::ToSink(msg)) => {
                            if !self.handle_msg(msg, &callback) {
                                break;
                            }
                        }
                        Err(_e) => {
                            println!("recv sees that all clients have closed");
                            break;
                        }
                    }
                }
            }
        }
    }

    fn handle_msg(
        &mut self,
        msg: shared::SinkMessage,
        callback: &mpsc::Sender<shared::BackendToGUIMessage>,
    ) -> bool {
        println!("SINK:\t handling resp: {:?}", msg);
        match msg {
            shared::SinkMessage::PlayButton => {
                if self.manual_sink_status.is_some() {
                    self.manual_sink_status = Some(true);
                }
                self.sink.play();
                callback
                    .send(shared::BackendToGUIMessage::SinkReports(
                        shared::SinkCallbackMessage::Playing,
                    ))
                    .unwrap();
                true
            }
            shared::SinkMessage::PauseButton => {
                if self.manual_sink_status.is_some() {
                    self.manual_sink_status = Some(false);
                }
                self.sink.pause();
                callback
                    .send(shared::BackendToGUIMessage::SinkReports(
                        shared::SinkCallbackMessage::Paused,
                    ))
                    .unwrap();
                true
            }
            shared::SinkMessage::LoadSong(path, volume) => {
                self.manual_sink_status = Some(true);
                self.sink.stop();
                self.sink = rodio::Sink::try_new(&self.stream_handle).unwrap();

                let file = io::BufReader::new(fs::File::open(path).unwrap());
                self.sink.append(rodio::Decoder::new(file).unwrap());
                self.sink.set_volume(volume);
                self.sink.play();
                self.time_elapsed = 0;
                callback
                    .send(shared::BackendToGUIMessage::SinkReports(
                        shared::SinkCallbackMessage::Playing,
                    ))
                    .unwrap();
                true
            }
            shared::SinkMessage::SetVolume(new_amount) => {
                self.sink.set_volume(new_amount);
                //callback.send(shared::SinkCallbackMessage::Playing).unwrap();
                true
            }
            shared::SinkMessage::Close => {
                self.sink.stop();
                false
            }
        }
    }

    fn handle_timeout(&mut self, callback: &mpsc::Sender<shared::BackendToGUIMessage>) {
        if self.manual_sink_status.is_some() && self.sink.len() == 0 {
            self.manual_sink_status = None;
            println!("SINK:\ttimeout on recv poll and we noticed the song was over");
            callback
                .send(shared::BackendToGUIMessage::SinkReports(
                    shared::SinkCallbackMessage::SongEnded,
                ))
                .unwrap();
        } else {
            match self.manual_sink_status {
                Some(true) => {
                    let new_time_elapsed = self.time_elapsed + BLOCKING_TIMEOUT;
                    if self.time_elapsed / 1000 != new_time_elapsed / 1000 {
                        callback
                            .send(shared::BackendToGUIMessage::SinkReports(
                                shared::SinkCallbackMessage::SecondElapsed,
                            ))
                            .unwrap();
                    }
                    self.time_elapsed = new_time_elapsed;
                }
                _ => (),
            };
            println!(
                "SINK:\tboring, timeout on recv poll and time passed is {}.{}",
                self.time_elapsed / 1000,
                self.time_elapsed % 1000
            );
        }
    }
}
