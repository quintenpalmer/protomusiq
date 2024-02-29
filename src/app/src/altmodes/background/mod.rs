use tiny_http::{Method, Response, Server};

use std::path;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::shared::{self, SinkCallbackMessage, SinkMessage};

use crate::services::sink;

#[derive(Debug)]
pub enum Error {
    ServerFinished,
}

pub fn run_server() -> Result<(), Error> {
    let (sink_client, sink_callback) = sink::create_backend_with_client_and_callback();

    let server = Server::http("0.0.0.0:5269").unwrap();

    loop {
        let maybe_req = server.try_recv().unwrap();
        let do_close = match maybe_req {
            None => {
                // Nothing to do if try_recv returns nothing yet
                // and "return" false to say that we should not close
                false
            }
            Some(mut req) => {
                let m_response = match req.url() {
                    "/play" => match req.method() {
                        Method::Post => {
                            sink_client.send(SinkMessage::PlayButton).unwrap();
                            Some(Response::from_string("let's play"))
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    "/pause" => match req.method() {
                        Method::Post => {
                            sink_client.send(SinkMessage::PauseButton).unwrap();
                            Some(Response::from_string("let's pause"))
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    "/load" => match req.method() {
                        Method::Post => {
                            let mut content = String::new();
                            req.as_reader().read_to_string(&mut content).unwrap();
                            println!("content: {}", content);
                            let lines: Vec<&str> = content.split('\n').collect();
                            println!("lines: {:?}", lines);
                            let filename = lines[0];
                            let raw_next_thing = lines[1];
                            let volume = lines[2];

                            let next_thing = if raw_next_thing == "none" {
                                None
                            } else if raw_next_thing == "pause" {
                                Some(shared::TrackPathOrPause::Pause)
                            } else {
                                Some(shared::TrackPathOrPause::TrackPath(path::PathBuf::from(
                                    raw_next_thing,
                                )))
                            };

                            sink_client
                                .send(SinkMessage::LoadSong(
                                    path::PathBuf::from(filename),
                                    next_thing,
                                    volume.parse::<f32>().unwrap(),
                                ))
                                .unwrap();

                            Some(Response::from_string("let's load"))
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    "/loadnextsong" => match req.method() {
                        Method::Post => {
                            let mut content = String::new();
                            req.as_reader().read_to_string(&mut content).unwrap();
                            println!("content: {}", content);
                            let lines: Vec<&str> = content.split('\n').collect();
                            println!("lines: {:?}", lines);
                            let raw_next_thing = lines[0];
                            let volume = lines[1];

                            let next_thing = if raw_next_thing == "none" {
                                None
                            } else if raw_next_thing == "pause" {
                                Some(shared::TrackPathOrPause::Pause)
                            } else {
                                Some(shared::TrackPathOrPause::TrackPath(path::PathBuf::from(
                                    raw_next_thing,
                                )))
                            };

                            sink_client
                                .send(SinkMessage::LoadNextSong(
                                    next_thing,
                                    volume.parse::<f32>().unwrap(),
                                ))
                                .unwrap();

                            Some(Response::from_string("let's load the next"))
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    "/setnextsong" => match req.method() {
                        Method::Post => {
                            let mut content = String::new();
                            req.as_reader().read_to_string(&mut content).unwrap();
                            let lines: Vec<&str> = content.split('\n').collect();
                            let filename = lines[0];

                            let msg = if filename == "pause" {
                                shared::TrackPathOrPause::Pause
                            } else {
                                shared::TrackPathOrPause::TrackPath(path::PathBuf::from(filename))
                            };

                            sink_client.send(SinkMessage::SetNextSong(msg)).unwrap();

                            Some(Response::from_string("let's set the next song"))
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    "/volume" => match req.method() {
                        Method::Post => {
                            let mut content = String::new();
                            req.as_reader().read_to_string(&mut content).unwrap();

                            sink_client
                                .send(SinkMessage::SetVolume(content.parse::<f32>().unwrap()))
                                .unwrap();

                            Some(Response::from_string("let's set the volume"))
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    "/close" => match req.method() {
                        Method::Post => {
                            sink_client.send(SinkMessage::Close).unwrap();
                            None
                        }
                        _ => Some(Response::from_string("method not allowed")),
                    },
                    _ => Some(Response::from_string("not found")),
                };

                let (close, response) = match m_response {
                    Some(r) => (false, r),
                    None => (true, Response::from_string("let's close")),
                };
                match req.respond(response) {
                    Ok(_) => println!("sent response"),
                    Err(_) => println!("could not send response"),
                };

                close
            }
        };

        let close_from_callback = match sink_callback.try_recv() {
            Ok(callback_msg) => {
                match callback_msg {
                    SinkCallbackMessage::Playing => {
                        ureq::post("http://localhost:5270/playing").call().unwrap();
                    }
                    SinkCallbackMessage::Paused => {
                        ureq::post("http://localhost:5270/paused").call().unwrap();
                    }
                    SinkCallbackMessage::SecondElapsed => {
                        ureq::post("http://localhost:5270/second_elapsed")
                            .call()
                            .unwrap();
                    }
                    SinkCallbackMessage::SongEnded => {
                        ureq::post("http://localhost:5270/song_ended")
                            .call()
                            .unwrap();
                    }
                };
                false
            }
            Err(mpsc::TryRecvError::Empty) => false,
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("recv sees that all clients have closed");
                true
            }
        };

        if do_close || close_from_callback {
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
    }

    Err(Error::ServerFinished)
}
