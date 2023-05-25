use iced::Command;

use crate::shared;

use super::init;
use super::message::{self, Message};
use super::state::{self, App, AppState};

mod action;
mod common;
mod nav;
mod playback;
mod sink;
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
        Message::PlaybackRequest(internal) => playback::handle_playback_request(app, internal),
        Message::SinkCallback(callback) => sink::handle_sink_callback(app, callback),
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
