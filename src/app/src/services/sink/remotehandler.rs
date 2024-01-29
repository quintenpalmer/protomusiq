use tiny_http::{Method, Response, Server};

use crate::shared;

#[derive(Debug)]
pub enum RemoteCallbackError {
    UnknownEndpoint,
    MethodNotAllowed,
}

pub struct CallbackServer {
    inner_server: Server,
}

impl CallbackServer {
    pub fn new() -> Self {
        let server = Server::http("0.0.0.0:5270").unwrap();

        CallbackServer {
            inner_server: server,
        }
    }

    pub fn try_recv(&self) -> Option<Result<shared::SinkCallbackMessage, RemoteCallbackError>> {
        let maybe_req = self.inner_server.try_recv().unwrap();
        match maybe_req {
            None => {
                // Nothing to do if try_recv returns nothing yet
                // and "return" false to say that we should not close
                None
            }
            Some(req) => {
                let m_response = match req.url() {
                    "/playing" => match req.method() {
                        Method::Post => Some(Ok(shared::SinkCallbackMessage::Playing)),
                        _ => Some(Err(RemoteCallbackError::MethodNotAllowed)),
                    },
                    "/paused" => match req.method() {
                        Method::Post => Some(Ok(shared::SinkCallbackMessage::Paused)),
                        _ => Some(Err(RemoteCallbackError::MethodNotAllowed)),
                    },
                    "/second_elapsed" => match req.method() {
                        Method::Post => Some(Ok(shared::SinkCallbackMessage::SecondElapsed)),
                        _ => Some(Err(RemoteCallbackError::MethodNotAllowed)),
                    },
                    "/song_ended" => match req.method() {
                        Method::Post => Some(Ok(shared::SinkCallbackMessage::SongEnded)),
                        _ => Some(Err(RemoteCallbackError::MethodNotAllowed)),
                    },
                    _ => Some(Err(RemoteCallbackError::UnknownEndpoint)),
                };

                let resp_payload = match m_response {
                    Some(Ok(ref _msg)) => Some(Response::from_string("responding ok")),
                    Some(Err(ref _e)) => Some(Response::from_string("encountered an error")),
                    None => None,
                };

                match resp_payload {
                    Some(p) => match req.respond(p) {
                        Ok(_) => println!("sent response"),
                        Err(_) => println!("could not send response"),
                    },
                    None => (),
                }
                m_response
            }
        }
    }
}
