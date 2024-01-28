use tiny_http::{Method, Response, Server};

use std::path;

use crate::shared::SinkMessage;

use crate::services::sink;

#[derive(Debug)]
pub enum Error {
    ServerFinished,
}

pub fn run_server() -> Result<(), Error> {
    let (sink_client, _sink_callback) = sink::create_backend_with_client_and_callback();

    let server = Server::http("0.0.0.0:5269").unwrap();

    loop {
        let mut req = server.recv().unwrap();
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
                    let volume = lines[1];

                    sink_client
                        .send(SinkMessage::LoadSong(
                            path::PathBuf::from(filename),
                            volume.parse::<f32>().unwrap(),
                        ))
                        .unwrap();

                    Some(Response::from_string("let's load"))
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

        if close {
            break;
        }
    }

    Err(Error::ServerFinished)
}
